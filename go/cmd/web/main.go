package main

import (
	"bufio"
	"bytes"
	"context"
	"encoding/base64"
	"flag"
	"fmt"
	"html"
	"io"
	"io/ioutil"
	"net/http"
	"os"
	"os/exec"
	"strings"
	"sync"

	_ "github.com/go-sql-driver/mysql"
	"github.com/golang/glog"
)

type Response struct {
	Status  string `json:"status"`
	Message string `json:"message"`
}

type Game struct {
	cmd    *exec.Cmd
	stdin  io.WriteCloser
	stdout *bufio.Reader
	stderr *bufio.Reader
	output chan string
}

var game Game
var m sync.Mutex

func handle(w http.ResponseWriter, r *http.Request) {
	if r.URL.Path != "/" {
		return
	}
	m.Lock()
	glog.Info("Request is starting...")
	defer func() {
		m.Unlock()
		glog.Info("Request finished.")
	}()
	r.ParseForm()
	w.Header().Set("Content-Type", "text/html")
	input := r.PostForm.Get("input")
	glog.Infof("Input: %s", input)
	if _, err := game.stdin.Write([]byte(strings.Trim(input, " \n") + "\n")); err != nil {
		glog.Fatalf("Error: %w", err)
	}

	buf := &bytes.Buffer{}
	var info string
	for {
		line := <-game.output
		if strings.HasPrefix(line, "###GUI###") {
			info = strings.TrimPrefix(line, "###GUI###")
			break
		}
		fmt.Fprintf(buf, "%s", html.EscapeString(line))
	}
	kvs := map[string]string{}
	for _, p := range strings.Split(info, "\t") {
		kv := strings.SplitN(p, ":", 2)
		if len(kv) != 2 {
			continue
		}
		kvs[kv[0]] = kv[1]
	}

	bw := bufio.NewWriter(w)
	fmt.Fprintf(bw, `<html><head><script
  src="https://code.jquery.com/jquery-3.5.1.min.js"
  integrity="sha256-9/aliU8dGd2tb6OSsuzixeV4y/faTqgFtohetphbbj0="
  crossorigin="anonymous"></script></head><body>`)
	fmt.Fprintf(bw, `<script>
		screen_x=%s;
		screen_y=%s;
		$(document).ready(function() {
		$('#screen').click(function(e) {
				var screen = $(this)[0];
				var offset = $(this).offset();
				var x = (e.pageX - offset.left) / screen.width * screen.naturalWidth - screen_x;
				var y = (e.pageY - offset.top) / screen.height * screen.naturalHeight - screen_y;
				$("input[name=input]")[0].value = Math.floor(x) + " " + Math.floor(y);
				$("form")[0].submit();
			});
		});
	</script>`, kvs["x"], kvs["y"])
	fmt.Fprintf(bw, "INFO: %s<br>", info)
	fmt.Fprintf(bw, "KVs: %v<br>", kvs)
	fmt.Fprintf(bw, `
		<form action="/" method="POST">
		Coordinates: <input name="input" type="text" value="">
		<input type=submit value="Send">
		<input type=button value="Undo" onclick="$('input[name=input]')[0].value = 'undo'; $('form')[0].submit();">
		</form>
	`)
	png, _ := os.Open("out/raw.png")
	img, _ := ioutil.ReadAll(png)
	dataURI := "data:image/png;base64," + base64.StdEncoding.EncodeToString(img)
	fmt.Fprintf(bw, `<img id="screen" src="%s" width="800px" style="image-rendering:pixelated">`, dataURI)
	fmt.Fprintf(bw, `<pre style="white-space: pre-wrap; font-family: 'Andale Mono', monospace;">`)
	fmt.Fprintf(bw, "%s", buf.String())
	fmt.Fprintf(bw, "</pre>")
	bw.Flush()
}

func main() {
	flag.Parse()
	var err error
	game.cmd = exec.CommandContext(context.Background(), flag.Args()[0], flag.Args()[1:]...)
	game.stdin, err = game.cmd.StdinPipe()
	if err != nil {
		glog.Fatalf("Failed to get stdin pipe: %w", err)
	}
	game.output = make(chan string, 10)

	stdout, err := game.cmd.StdoutPipe()
	if err != nil {
		glog.Fatalf("Failed to get stdout pipe: %w", err)
	}
	go func() {
		game.stdout = bufio.NewReaderSize(stdout, 16 * 1024 * 1024)
		for {
			buf, err := game.stdout.ReadSlice('\n')
			if err != nil {
				glog.Fatalf("Failed to read from stdout: %w", err)
			}
			glog.Infof("STDOUT: %s", string(buf))
			game.output <- string(buf)
		}
	}()

	stderr, err := game.cmd.StderrPipe()
	if err != nil {
		glog.Fatalf("Failed to get stderr pipe: %w", err)
	}
	go func() {
		game.stderr = bufio.NewReaderSize(stderr, 16 * 1024 * 1024)
		for {

			buf, err := game.stderr.ReadSlice('\n')
			if err != nil {
				glog.Fatalf("Failed to read from stderr: %w", err)
			}
			glog.Infof("STDERR: %s", string(buf))
			game.output <- string(buf)
		}
	}()

	game.cmd.Start()

	// 最初は捨てる
	for {
		line := <-game.output
		if strings.HasPrefix(line, "###GUI###") {
			break
		}
	}

	http.HandleFunc("/", handle)
	http.Handle("/out/", http.StripPrefix("/out/", http.FileServer(http.Dir("out"))))
	addr := os.Getenv("GUI_ADDRESS")
	if addr == "" {
		addr = ":8001"
	}
	glog.Infof("Starting server (%s)...", addr)
	http.ListenAndServe(addr, nil)
}
