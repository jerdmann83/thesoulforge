package poller

import (
	"fmt"
	"net/http"
	"time"
)

type Result struct {
	Error   error
	Latency time.Duration
	Ep      Pollable
}

type Pollable interface {
	Name() string
	Url() string
}

func Poll(ep Pollable, out chan Result, interval time.Duration) {
	ticker := time.NewTicker(interval)
	for range ticker.C {
		st := time.Now()
		resp, err := http.Get(ep.Url())
		if err != nil {
			fmt.Println(ep.Url(), " failed: ", err)
			continue
		}
		latency := time.Now().Sub(st)
		out <- Result{
			err,
			latency,
			ep}
		resp.Body.Close()
	}
}
