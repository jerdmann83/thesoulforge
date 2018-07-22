package main

import (
	"encoding/hex"
	"flag"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"strconv"
	"time"
)

const DEFAULT_BYTES = 1024

var hostname, _ = os.Hostname()

var logger = log.New(os.Stdout, hostname+" ", log.LstdFlags)

type Config struct {
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
		err := r.ParseForm()
		if err != nil {
			handleError(w, err, 501)
			return
		}

		rsize, _ := strconv.Atoi(r.Form.Get("bytes"))
		if rsize == 0 {
			rsize = DEFAULT_BYTES
		}

		buf := make([]byte, rsize)
		bytes, err := rd.Read(buf)
		if err != nil {
			handleError(w, err, 502)
			return
		}

		logger.Printf("%v bytes err=%v", bytes, err)
		var s string
		binaryOut, _ := strconv.ParseBool(r.Form.Get("binary"))
		if binaryOut {
			s = string(buf)
		} else {
			s = hex.Dump(buf)
		}
		fmt.Fprintf(w, s)
	}
}

func main() {
	var port uint
	var datasrc string
	var binary bool
	flag.UintVar(&port, "port", 8080, "port")
	flag.StringVar(&datasrc, "datasrc", "/dev/urandom", "/data bytes source")
	flag.BoolVar(&binary, "binary", false, "send raw binary data")
	flag.Parse()

	http.HandleFunc("/", rootHandler)
	http.HandleFunc("/health", healthHandler)

	c := Config{binary}
	src, err := os.Open(datasrc)
	if err != nil {
		log.Fatal(err)
		return
	}
	defer src.Close()
	http.HandleFunc("/data", makeHandler(c, src))

	addr := fmt.Sprintf(":%v", port)
	log.Fatal(http.ListenAndServe(addr, nil))
}
