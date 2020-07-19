package main

import (
	"fmt"
	"log"
	"net/http"
	"net/http/httputil"
)

func main() {
	director := func(req *http.Request) {
		url := *req.URL
		url.Scheme = "https"
		url.Host = "icfpc2020-api.testkontur.ru"
		req.Host = url.Host
		req.URL = &url
		buf, _ := httputil.DumpRequest(req, false)
		fmt.Println(string(buf))
		// buffer, err := ioutil.ReadAll(request.Body)
		// if err != nil {
		// 	log.Fatal(err.Error())
		// }
		// req, err := http.NewRequest(request.Method, url.String(), bytes.NewBuffer(buffer))
		// if err != nil {
		// 	log.Fatal(err.Error())
		// }
		// req.Header = request.Header
		// *request = *req
	}
	modifier := func(resp *http.Response) error {
		buf, _ := httputil.DumpResponse(resp, true)
		fmt.Println(string(buf))
		return nil
	}
	rp := &httputil.ReverseProxy{
		Director:       director,
		ModifyResponse: modifier,
	}
	server := http.Server{
		Addr:    ":9000",
		Handler: rp,
	}
	fmt.Println("Started...")
	if err := server.ListenAndServe(); err != nil {
		log.Fatal(err.Error())
	}
}
