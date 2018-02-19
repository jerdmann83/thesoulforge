package poller

import (
	"fmt"
	"net/http"
	"time"
)

type Result struct {
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
		_, err := http.Get(ep.Url())
		if err != nil {
			fmt.Println(ep.Url(), " fail")
			continue
		}
		latency := time.Now().Sub(st)
		out <- Result{
			latency,
			ep}
	}
}
