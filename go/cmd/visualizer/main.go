package main

import (
	"bufio"
	"context"
	"encoding/json"
	"flag"
	"fmt"
	"io"
	"net/http"
	"os"
	"os/exec"
	"regexp"
	"strconv"
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

type JsonObject struct {
	Array []JsonObject
	Int		int64
	String string
	Unknown interface{}
}

func (j *JsonObject) ToList() []int64 {
	xs := []int64{}
	for _, x := range j.Array {
		xs = append(xs, x.Int)
	}
	return xs
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


type X2 struct {
	TotalCost int64 `json:"total_cost"`
	MaxAccel int64 `json:"max_accel"`
	MaxTemp int64 `json:"max_temp"`
}

func ParseX2(obj JsonObject) (X2, error) {
	r := X2{}
	if len(obj.Array) != 3 {
		return r, fmt.Errorf("invalid x2: %d", len(obj.Array))
	}
	r.TotalCost = obj.Array[0].Int
	r.MaxAccel = obj.Array[1].Int
	r.MaxTemp = obj.Array[2].Int
	return r, nil
}

type X3 struct {
	RangeMin int64 `json:"range_min"`
	RangeMax int64 `json:"range_max"`
}

func ParseX3(obj JsonObject) (X3, error) {
	r := X3{}
	if len(obj.Array) != 2 {
		return r, fmt.Errorf("invalid x3: %d", len(obj.Array))
	}
	r.RangeMin = obj.Array[0].Int
	r.RangeMax = obj.Array[1].Int
	return r, nil
}

type X4 struct {
	Used bool `json:"used"`
	X0 int64 `json:"x0"`
	X1 X3 `json:"x1"`
	X2 X2 `json:"x2"`
	X3 X3 `json:"x3"`
}

func ParseX4(obj JsonObject) (X4, error) {
	r := X4{}
	if len(obj.Array) == 0 {
		return r, nil
	}
	if len(obj.Array) != 4 {
		return r, fmt.Errorf("invalid x4: %d", len(obj.Array))
	}
	var err error
	r.Used = true
	r.X0 = obj.Array[0].Int
	r.X1, err = ParseX3(obj)
	if err != nil {
		return r, err
	}
	r.X2, err = ParseX2(obj)
	if err != nil {
		return r, err
	}
	r.X3, err = ParseX3(obj)
	if err != nil {
		return r, err
	}
	return r, nil
}

type StaticGameInfo struct {
	X0 int64 `json:"x0"`
	Role int64 `json:"role"`
	X2 X2 `json:"x2"`
	X3 X3 `json:"x3"`
	X4 X4 `json:"x4"`
}

func ParseStaticGameInfo(obj JsonObject) (StaticGameInfo, error) {
	r := StaticGameInfo{}
	if len(obj.Array) != 5 {
		return r, fmt.Errorf("invalid static_game_info: %d", len(obj.Array))
	}
	var err error
	r.X0 = obj.Array[0].Int
	r.Role = obj.Array[1].Int
	r.X2, err = ParseX2(obj.Array[2])
	if err != nil {
		return r, err
	}
	r.X3, err = ParseX3(obj.Array[3])
	if err != nil {
		return r, err
	}
	//r.X4, err = ParseX4(obj.Array[3])
	//if err != nil {
	//	return r, err
	//}
	return r, nil
}

type GameState struct {
	GameTick int64 `json:"game_tick"`
	X1 X3 `json:"x1"`
	
}

type Response struct {
	GameStage int64 `json:"game_stage"`
	StaticGameInfo StaticGameInfo `json:"static_game_info"`
}

func ParseResponse(obj JsonObject) (Response, error) {
	r := Response{}
	if len(obj.Array) != 4 {
		return r, fmt.Errorf("invalid response")
	}
	var err error
	r.GameStage = obj.Array[1].Int
	r.StaticGameInfo, err = ParseStaticGameInfo(obj.Array[2])
	if err != nil {
		return r, err
	}
	return r, nil
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

		// {"ship":{"role":1,"shipId":0,"position":{"x":-5,"y":-29},"velocity":{"x":-2,"y":7},"x4":[7,1,1,3],"x5":60,"x6":64,"x7":1},"appliedCommands":[]}


		fmt.Fprintf(w, "[")
		for shipID, msgs := range ships {
			for _, msg := range msgs {
				m := strings.SplitN(msg, "\t", 2)
				if m[0] == "RESP" {
					//glog.Infof("RESP: %s", m[1])
					jo := Parse(m[1])
					obj, err := ParseResponse(jo)
					if err != nil {
						glog.Errorf("%v", err)
					}
					buf, err := json.Marshal(obj)
					if err != nil {
						glog.Errorf("%v", err)
					}
					glog.Infof("RESP: %s", string(buf))
				} else if m[0] == "SEND" {
					glog.Infof("%v", Parse(m[1]))
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

func ParseObject(x interface{}) JsonObject {
	if xs, ok := x.([]interface{}); ok {
		ys := make([]JsonObject, 0)
		for _, x := range xs {
			ys = append(ys, ParseObject(x))
		}
		return JsonObject{Array: ys}
	}
	if xs, ok := x.(string); ok {
		if i, err := strconv.ParseInt(xs, 10, 64); err == nil {
			return JsonObject{Int: i}
		}
		return JsonObject{String: xs}
	}
	return JsonObject{Unknown: x}
}

func Parse(s string) JsonObject {
	s = regexp.MustCompile(`-?\d+`).ReplaceAllString(s, `"$0"`)
	s = strings.ReplaceAll(s, "<", "[")
	s = strings.ReplaceAll(s, ">", "]")
	glog.Info(s)
	var v interface{}
	if err := json.Unmarshal([]byte(s), &v); err != nil {
		glog.Errorf("failed to parse JSON: %v", err)
	}
	return ParseObject(v)
}

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
