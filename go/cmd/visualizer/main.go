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

//type Response struct {
//	Status  string `json:"status"`
//	Message string `json:"message"`
//}

type Game struct {
	cmd    *exec.Cmd
	stdin  io.WriteCloser
	stdout *bufio.Reader
	stderr *bufio.Reader
	output chan string
}

var game Game
var output = make(chan string, 10)
var outputPool = make([]string, 0)
var m sync.RWMutex

//#[derive(Debug, Clone)]
//pub struct Response {
//pub stage: i32,
//pub info: Info,
//pub state: State,
//}
//
//fn response_to_json(x: &Response) -> String {
//format!("{{\"stage\":{},\"state\":{}}}", x.stage, state_to_json(&x.state))
//}

type Response struct {
	Stage int64 `json:"stage"`
	State State `json:"state"`
}

//#[derive(Debug, Clone)]
//pub struct State {
//pub tick: i32,
//pub range: Range<i32>, // 侵入可能エリアの x,y の絶対値の範囲
//pub ships: Vec<Ship>,
//}
//
//fn state_to_json(x: &State) -> String {
//let mut ships = Vec::new();
//for s in &x.ships {
//ships.push(ship_to_json(&s));
//}
//format!("{{\"ships\":[{}]}}", ships.connect(","))
//}

type State struct {
	Tick  int64  `json:"tick"`
	Ships []Ship `json:"ships"`
}

//#[derive(Debug, Clone)]
//pub struct Ship {
//pub role: i32,
//pub id: i32,
//pub pos: (i32, i32),
//pub v: (i32, i32),
//pub status: Params,
//pub heat: i32,
//pub max_heat: i32,
//pub max_accelarate: i32,
//pub commands: Vec<Command>,
//}
//
//fn ship_to_json(x: &Ship) -> String {
//format!("{{\"role\":{},\"x\":{},\"y\":{}}}", x.role, x.pos.0, x.pos.1)
//}

type Ship struct {
	Role int64 `json:"role"`
	X    int64 `json:"x"`
	Y    int64 `json:"y"`
}

func handle(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, `
		<html>
			<head>
				<script
					src="https://code.jquery.com/jquery-3.5.1.min.js"
  				integrity="sha256-9/aliU8dGd2tb6OSsuzixeV4y/faTqgFtohetphbbj0="
					crossorigin="anonymous"></script>
<script>

var last_data = "";
var responses = [];

// 画面のサイズ
var scale = 256;

$(function(){

const canvas = document.getElementById('canvas');
const c = canvas.getContext('2d');

function init() {
	update();
	//setInterval(function(){ update() }, 1000);
}

function update() {
	$.get("output.txt", function(data){
		if (last_data != data) {
			last_data = data;
			//$("#commands").text(data);
			responses = $.parseJSON(data);

			// スケールの計算
			var max_value = 0;
			for (var i = 0; i < responses.length; i++) {
				var ships = responses[i]["response"]["state"]["ships"];
				for (var j = 0; j < ships.length; j++) {
					max_value = Math.max(max_value, Math.abs(ships[j]["x"]), Math.abs(ships[j]["y"]));
				}
			}

			//scale = max_value * 2.5;
			update_frame();
		}
	});
}

$(window).keydown(function(e){
	console.log("keydown: " + e.keyCode);
	var frame = $("#frame").text();

	// LEFT
	if (e.keyCode == 37 || e.keyCode == 65) {
		$("#frame").text($("#frame").text() - 1);
	}
	// RIGHT
	if (e.keyCode == 39 || e.keyCode == 68) {
		$("#frame").text($("#frame").text() - 0 + 1);
	}

	if ($("#frame").text() - 0 < 0) {
		$("#frame").text("0");
	}
	if ($("#frame").text() - 0 >= responses.length) {
		$("#frame").text(responses.length - 1);
	}

	update_frame();
});

function update_frame() {
	var frame = $("#frame").text() - 0;
	var r = responses[frame];
	var response = responses[frame]["response"];
	$("#command").text(JSON.stringify(response, null, "  "));
	draw(response);
}

function to_canvas_x(x) {
	return x / scale * canvas.width + canvas.width / 2;
}

function to_canvas_y(y) {
	return y / scale * canvas.height + canvas.height / 2;
}

function draw(response) {
	clear();

	var ships = response["state"]["ships"];
	const canvas = document.getElementById('canvas');

	// Detonate
	for (var i = 0; i < ships.length; i++) {
		var ship = ships[i];
		var commands = ship["commands"];
		for (var j = 0; j < commands.length; j++) {
			var command = commands[j];
			if (command["type"] == "detonate") {
				rect(
					to_canvas_x(ship["x"] - 10),
					to_canvas_y(ship["y"] - 10),
					to_canvas_x(ship["x"] + 10),
					to_canvas_y(ship["y"] + 10),
					"#ff88ff",
				);
			}
		}
	}

	// Ships
	for (var i = 0; i < ships.length; i++) {
		var ship = ships[i];
		color = "#ff8800";
		if (ship["role"] == 1) {
			color = "#0088ff";
		}
		circle(
			ship["x"] / scale * canvas.width + canvas.width / 2,
			ship["y"] / scale * canvas.height + canvas.height / 2,
			10, color);
	}

	// Beam
	for (var i = 0; i < ships.length; i++) {
		var ship = ships[i];
		color = "#ff8800";
		if (ship["role"] == 1) {
			color = "#0088ff";
		}
		var commands = ship["commands"];
		for (var j = 0; j < commands.length; j++) {
			var command = commands[j];
			if (command["type"] == "shoot") {
				circle(
					to_canvas_x(ship["x"]),
					to_canvas_y(ship["y"]),
					5, "red");
				line(
					to_canvas_x(ship["x"]),
					to_canvas_y(ship["y"]),
					to_canvas_x(command["x"]),
					to_canvas_y(command["y"]),
					color,
				);
			}
		}
	}

	// The Sun
	c.beginPath();
	c.rect(
		 canvas.width / 2 - 16 / scale * canvas.width,
		 canvas.height / 2 - 16 / scale * canvas.height,
		 32 / scale * canvas.width,
		 32 / scale * canvas.height,
	);
	c.fillStyle = "#ffeeaa";
	c.strokeStyle = c.fillStyle;
	c.lineWidth = 1;
	c.fill();
	c.stroke();
}

function line(x1, y1, x2, y2, color) {
	c.strokeStyle = color;
	c.lineWidth = 3;
	c.beginPath();
	c.moveTo(x1, y1);
	c.lineTo(x2, y2);
	c.stroke();
}

function circle(x, y, r, color) {
	c.beginPath();
	c.arc(x, y, r, 0 * Math.PI / 180, 360 * Math.PI / 180, false);
	c.fillStyle = color;
	c.fill();
	c.strokeStyle = color;
	c.lineWidth = 1;
	c.stroke();
}

function rect(x1, y1, x2, y2, color) {
	c.beginPath();
	c.fillStyle = color;
	c.strokeStyle = color;
	c.rect(x1, y1, x2 - x1, y2 - y1);
	c.lineWidth = 1;
	c.fill();
	c.stroke();
}

function clear() {
	c.beginPath();
	//c.clearRect(0, 0, canvas.width, canvas.height);
	c.fillStyle = "#000022";
	c.rect(0, 0, canvas.width, canvas.height);
	c.fill();
	c.stroke();
}

init();

});

</script></head><body style="background: #000; color: #fff">
<div style="margin: auto auto; text-align: center;">
	<div>Frame: <span id="frame">0</span></div>
	<canvas id="canvas" width="500" height="500" style="border:1px solid #888"></canvas>
</div>
<pre id="command" style="font-family: 'Andale Mono', monospace"></pre>`)
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

		fmt.Fprint(w, "[")
		for i, line := range output {
			//			###GUI TIME ID KEY MSG
			row := strings.SplitN(strings.TrimRight(line, "\r\n"), "\t", 5)
			if len(row) != 5 {
				glog.Errorf("Invalid line: %s", line)
				continue
			}
			if i != 0 {
				fmt.Fprintf(w, ",\n")
			}
			fmt.Fprintf(w, `{"response":%s}`, row[4])
		}
		fmt.Fprint(w, "]")
	})

	addr := os.Getenv("GUI_ADDRESS")
	if addr == "" {
		addr = ":8001"
	}
	glog.Infof("Starting server (%s)...", addr)
	http.ListenAndServe(addr, nil)
}
