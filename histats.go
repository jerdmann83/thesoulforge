package main

import (
	"fmt"
	"log"
	"net/http"
	"time"
)

// # HELP process_virtual_memory_bytes Virtual memory size in bytes.
// # TYPE process_virtual_memory_bytes gauge
// process_virtual_memory_bytes 8.68343808e+08

type Metric struct {
	timestamp int64
	value     float64
}

type Series struct {
	name, help, metricType string
	values                 []Metric
}

type Endpoint struct {
	url string
}

func floatMillisecond(t time.Duration) float64 {
	return float64(t/time.Microsecond) / 1000
}

func pollEndpoint(out chan Metric, ep Endpoint, interval time.Duration) {
	ticker := time.NewTicker(interval)
	for range ticker.C {
		st := time.Now()
		_, err := http.Get(ep.url)
		if err != nil {
			latency := time.Now().Sub(st)
			out <- Metric{
				time.Now().Unix(),
				floatMillisecond(latency)}
		}
	}
}

func dumpMetrics(w http.ResponseWriter, req *http.Request) {
	fmt.Fprintf(w, "rofl hi")
}

func collectSeries(in chan Metric) {
	for {
		s := <-in
		fmt.Printf("collect: got %s\n", s)
	}
}

func main() {
	endpoints := []string{"https://www.google.com"}
	out := make(chan Metric)

	go collectSeries(out)
	for _, url := range endpoints {
		go pollEndpoint(out, Endpoint{url}, 1*time.Second)
	}

	http.HandleFunc("/metrics", dumpMetrics)
	log.Fatal(http.ListenAndServe(":8081", nil))
}
