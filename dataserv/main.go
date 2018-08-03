package main

import (
	"flag"
	"fmt"
	"log"
	"net/http"
	"os"
)

func main() {
	var port uint
	var datasrc string
	flag.UintVar(&port, "port", 8080, "port")
	flag.StringVar(&datasrc, "datasrc", "/dev/urandom", "/data bytes source")
	flag.Parse()

	hostname, _ := os.Hostname()
	logger := log.New(os.Stdout, hostname+" ", log.LstdFlags)

	h := HandlerArgs{hostname, logger}

	http.HandleFunc("/", RootHandler(h))
	http.HandleFunc("/health", HealthHandler(h))

	src, err := os.Open(datasrc)
	if err != nil {
		log.Fatal(err)
		return
	}
	defer src.Close()
	http.HandleFunc("/data", DataHandler(h, src))

	addr := fmt.Sprintf(":%v", port)
	h.Printf("starting up %v", addr)
	log.Fatal(http.ListenAndServe(addr, nil))
}
