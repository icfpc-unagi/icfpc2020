package main

import (
	"bufio"
	"context"
	"flag"
	"fmt"
	"io"
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

type JsonObject struct {
	Array []JsonObject
	Int		int64
}

var game Game
var output = make(chan string, 10)
var outputPool = make([]string, 0)
var m sync.RWMutex

func handle(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, `<html><head><script
  src="https://code.jquery.com/jquery-3.5.1.min.js"
  integrity="sha256-9/aliU8dGd2tb6OSsuzixeV4y/faTqgFtohetphbbj0="
  crossorigin="anonymous"></script><script>
		var last_data = "";
		function update() {
				$.get("output.txt", function(data){
					if (last_data != data) {
						last_data = data;
						$("#commands").text(data);
					}
				});
		}
		$(function(){
			update();
			setInterval(function(){ update() }, 1000);
		});
	</script></head><body>
  <textarea id="commands"></textarea>`)
}

func main() {
	flag.Parse()
	for i, c := range flag.Args() {
		cmd := exec.CommandContext(context.Background(), "bash", "-c", c)
		cmd.Stderr = os.Stderr
		stdout, err := cmd.StdoutPipe()
		if err != nil {
			glog.Fatalf("Failed to get stdout pipe: %v", err)
		}
		go func() {
			r := bufio.NewReaderSize(stdout, 16*1024*1024)
			for {
				buf, err := r.ReadSlice('\n')
				if err != nil {
					glog.Errorf("#%d process exited", i)
					break
				}
				line := string(buf)
				if strings.HasPrefix(line, "###GUI") {
					output <- line
				}
			}
		}()
		cmd.Start()
	}

	go func() {
		for {
			line := <-output
			m.Lock()
			outputPool = append(outputPool, line)
			m.Unlock()
		}
	}()

	http.HandleFunc("/", handle)

	http.HandleFunc("/output.txt", func(w http.ResponseWriter, r *http.Request) {
		output := func() []string {
			m.RLock()
			defer m.RUnlock()
			lines := make([]string, 0)
			for _, line := range outputPool {
				lines = append(lines, line)
			}
			return lines
		}()

		w.Header().Set("Content-Type", "text/plain")

		ships := map[string][]string{}
		for _, line := range output {
			// ###GUI TIME ID MSG
			row := strings.SplitN(strings.TrimRight(line, "\r\n"), "\t", 4)
			if len(row) != 4 {
				glog.Errorf("Invalid line: %s", line)
				continue
			}
			ships[row[2]] = append(ships[row[2]], row[3])
		}

		fmt.Fprintf(w, "[")
		for shipID, msgs := range ships {
			for _, msg := range msgs {
				m := strings.SplitN(msg, "\t", 2)
				if m[0] == "SEND" || m[0] == "RESP" {
					m[1] = EToJson(m[1])
				}
				fmt.Fprintf(w, `{"playerKey": "%s", "type": "%s", "command": %s},`, shipID, m[0], m[1])
				fmt.Fprintf(w, "\n")
			}
		}
		fmt.Fprintf(w, "null]")
	})

	addr := os.Getenv("GUI_ADDRESS")
	if addr == "" {
		addr = ":8001"
	}
	glog.Infof("Starting server (%s)...", addr)
	http.ListenAndServe(addr, nil)
}

func Parse(s string) 

func EToJson(s string) string {
	s = strings.ReplaceAll(s, "[", "@LIST_LEFT@")
	s = strings.ReplaceAll(s, "]", "@LIST_RIGHT@")
	s = strings.ReplaceAll(s, "<", "@PAIR_LEFT@")
	s = strings.ReplaceAll(s, ">", "@PAIR_RIGHT@")
	s = strings.ReplaceAll(s, "@PAIR_LEFT@", "[")
	s = strings.ReplaceAll(s, "@PAIR_RIGHT@", "]")
	s = strings.ReplaceAll(s, "@LIST_LEFT@@LIST_RIGHT@", "[]")
	s = strings.ReplaceAll(s, "@LIST_LEFT@", "[")
	s = strings.ReplaceAll(s, "@LIST_RIGHT@", ", null]")
	// {"ship":{"role":1,"shipId":0,"position":{"x":-9,"y":-12},"velocity":{"x":-2,"y":9},"x4":[0,0,0,0],"x5":59,"x6":64,"x7":1},"appliedCommands":[]}
	// [1, 0, [256, 1, [448, 2, 128], [16, 128], []], []]
	//s, err := func() (string, error) {
	//	var v []interface{}
	//	if err := json.Unmarshal([]byte(s), &v); err != nil {
	//		return "", err
	//	}
	//	if len(v) != 4 {
	//		return "", fmt.Errorf("unexpected number of args: %v", v)
	//	}
	//	staticGameInfo, ok := v[2].([]interface{})
	//	if !ok {
	//		return "", fmt.Errorf("failed to parse staticGameInfo")
	//	}
	//	role, ok := staticGameInfo[1].(float64)
	//	if !ok {
	//		return "", fmt.Errorf("role is missing")
	//	}
	//	gameStage, ok := v[1].(float64)
	//	if !ok {
	//		return "", fmt.Errorf("role is missing")
	//	}
	//	vv := struct {
	//		Role float64 `json:"role"`
	//		GameStage float64 `json:"game_stage"`
	//	}{
	//		Role: role,
	//		GameStage: gameStage,
	//	}
	//	buf, err := json.Marshal(vv)
	//	if err != nil {
	//		return "", err
	//	}
	//	return string(buf), nil
	//}()
	//if err != nil {
	//	glog.Errorf("Failed to parse: %v: %s", err, s)
	//}
	return s
}
