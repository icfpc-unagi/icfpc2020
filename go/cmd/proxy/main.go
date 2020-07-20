package main

import (
	"bytes"
	"flag"
	"fmt"
	"io/ioutil"
	"net/http"

	"github.com/golang/glog"
)

var icfpcServerURL = flag.String("endpoint", "", "API endpoint")
var port = flag.String("port", ":9000", "API endpoint")

func handler(w http.ResponseWriter, r *http.Request) {
	glog.Info("Processing request...")
	if r.Body == nil {
		glog.Errorf("body is empty")
		w.WriteHeader(400)
		return
	}
	defer r.Body.Close()
	buf, err := ioutil.ReadAll(r.Body)
	if err != nil {
		glog.Errorf("body is broken: %#v", err)
		w.WriteHeader(400)
		return
	}
	fmt.Printf("REQUEST\t%s\n", string(buf))
	resp, err := http.Post(*icfpcServerURL, "text/plain", bytes.NewBuffer(buf))
	if err != nil {
		glog.Errorf("POST failed: %s", err)
		w.WriteHeader(500)
		return
	}
	if resp.Body == nil {
		glog.Errorf("response body is empty")
		w.WriteHeader(500)
		return
	}
	defer resp.Body.Close()
	buf, err = ioutil.ReadAll(resp.Body)
	if err != nil {
		glog.Errorf("failed to read response body: %#v", err)
		w.WriteHeader(500)
		return
	}
	fmt.Printf("RESPONSE\t%s\n", string(buf))
	w.Header().Set("Content-Type", "text/plain")
	if _, err := w.Write(buf); err != nil {
		glog.Errorf("failed to respond")
		w.WriteHeader(500)
		return
	}
}

func main() {
	flag.Parse()
	glog.Info("Initializing...")
	if *icfpcServerURL == "" {
		glog.Fatal("--endpoint flag is required")
	}
	http.HandleFunc("/", handler)
	glog.Infof("Starting server on %s...", *port)
	if err := http.ListenAndServe(*port, nil); err != nil {
		glog.Fatal(err.Error())
	}
}
