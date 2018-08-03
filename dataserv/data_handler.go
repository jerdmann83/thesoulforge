package main

import (
	"encoding/hex"
	"fmt"
	"io"
	"log"
	"net/http"
	"strconv"
	"time"
)

type HandlerArgs struct {
	hostname    string
	*log.Logger //anonymous
}

func handleError(w http.ResponseWriter, err error, responseCode int) {
	var s string
	fmt.Sprintf(s, "read failed: %v", err)
	//logger.Printf(s)
	http.Error(w, s, responseCode)
	w.WriteHeader(http.StatusInternalServerError)
}

func RootHandler(h HandlerArgs) func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		http.Redirect(w, r, "/health", http.StatusFound)
	}
}

func HealthHandler(h HandlerArgs) func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "%v | %v: OK", time.Now(), h.hostname)
	}
}

func DataHandler(h HandlerArgs, rd io.Reader) func(w http.ResponseWriter, r *http.Request) {
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

		h.Printf("%v bytes err=%v", bytes, err)
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
