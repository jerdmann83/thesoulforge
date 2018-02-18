package main

import (
	"fmt"
	"log"
	"net/http"
	"sync"
	"time"
)

type Metric struct {
	timestamp int64
	value     float64
}

type Series struct {
	name, help, metricType string
	latest                 Metric
}

type Endpoint struct {
	name, url string
}

type pollResult struct {
	metric Metric
	ep     Endpoint
}

type metricRepo struct {
	metrics map[string]Series
	sync.Mutex
}

func floatMillisecond(t time.Duration) float64 {
	return float64(t/time.Microsecond) / 1000
}

func pollEndpoint(out chan pollResult, ep Endpoint, interval time.Duration) {
	ticker := time.NewTicker(interval)
	for range ticker.C {
		st := time.Now()
		_, err := http.Get(ep.url)
		if err != nil {
			continue
		}
		latency := time.Now().Sub(st)
		out <- pollResult{
			Metric{
				time.Now().Unix(),
				floatMillisecond(latency)},
			ep}
	}
}

//func makeHandler(repo* metricRepo
var repo metricRepo

func dumpMetrics(w http.ResponseWriter, req *http.Request) {
	// # HELP process_virtual_memory_bytes Virtual memory size in bytes.
	// # TYPE process_virtual_memory_bytes gauge
	// process_virtual_memory_bytes 8.68343808e+08
	repo.Lock()
	defer repo.Unlock()
	for k, v := range repo.metrics {
		fmt.Fprintf(w, "%v_response_ms %v", k, v.latest.value)
	}

}

func collectSeries(in chan pollResult) {
	for {
		result := <-in
		repo.Lock()
		name := result.ep.name
		// TODO golang doesn't allow direct assignment of map struct members?
		// really awkward copy and reinsert for now
		updated := repo.metrics[name]
		updated.latest = result.metric
		repo.metrics[name] = updated
		repo.Unlock()
	}
}

func main() {
	// TODO no global
	//repo := metricRepo{
	//	metrics: make(map[string]Series),
	//}
	repo.metrics = make(map[string]Series)

	endpoints := map[string]string{
		"google": "https://www.google.com",
	}
	out := make(chan pollResult)

	go collectSeries(out)
	for name, url := range endpoints {
		go pollEndpoint(out, Endpoint{name, url}, 1*time.Second)
	}

	http.HandleFunc("/metrics", dumpMetrics)
	log.Fatal(http.ListenAndServe(":8081", nil))
}
