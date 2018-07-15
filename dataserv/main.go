package main

import (
	"encoding/hex"
	"flag"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"time"
)

var hostname, _ = os.Hostname()

var logger = log.New(os.Stdout, hostname+" ", log.LstdFlags)

//TODO: doesn't compile?
//const (
//Binary DataType = iota
//Hex
//)

type Config struct {
	datasize uint
	//rType    DataType
	binaryOut bool
}

func rootHandler(w http.ResponseWriter, r *http.Request) {
	http.Redirect(w, r, "/health", http.StatusFound)
}

func healthHandler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "%v | %v: OK", time.Now(), hostname)
}

func handleError(w http.ResponseWriter, err error, responseCode int) {
	var s string
	fmt.Sprintf(s, "read failed: %v", err)
	logger.Printf(s)
	http.Error(w, s, responseCode)
	w.WriteHeader(http.StatusInternalServerError)
}

func makeHandler(c Config, rd io.Reader) func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		buf := make([]byte, c.datasize)
		bytes, err := rd.Read(buf)
		if err != nil {
			handleError(w, err, 502)
			return
		}
		logger.Printf("read %v bytes", bytes)
		var s string
		if c.binaryOut {
			s = string(buf)
		} else {
			s = hex.Dump(buf)
		}
		fmt.Fprintf(w, s)
	}
}

func main() {
	var datasize uint
	var datasrc string
	var binary bool
	flag.UintVar(&datasize, "datasize", 1024, "/data endpoint response size")
	flag.StringVar(&datasrc, "datasrc", "/dev/random", "/data bytes source")
	flag.BoolVar(&binary, "binary", false, "send raw binary data")
	flag.Parse()

	http.HandleFunc("/", rootHandler)
	http.HandleFunc("/health", healthHandler)

	c := Config{datasize, binary}
	src, err := os.Open(datasrc)
	if err != nil {
		log.Fatal(err)
		return
	}
	http.HandleFunc("/data", makeHandler(c, src))
	log.Fatal(http.ListenAndServe(":8080", nil))
}
