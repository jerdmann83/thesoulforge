package main

import (
	"fmt"
	"github.com/jerdmann/thesoulforge/poller"
	"log"
	"net/http"
	"strings"
	"sync"
	"time"
)

func toMetric(t time.Duration) float64 {
	return float64(t/time.Microsecond) / 1000
}

type Metric struct {
	// timestamp int64
	value float64
}

type Series struct {
	name, help, metricType string
	latest                 Metric
}

type metricRepo struct {
	metrics map[string]Series
	sync.Mutex
}

type Endpoint struct {
	name, url string
}

func (ep Endpoint) Name() string {
	return ep.name
}

func (ep Endpoint) Url() string {
	return ep.url
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
		fmt.Fprintf(w, "%v_response_ms %v\n", k, v.latest.value)
	}

}

func collectSeries(in chan poller.Result) {
	for {
		result := <-in
		repo.Lock()
		name := result.Ep.Name()
		// TODO golang doesn't allow direct assignment of map struct members?
		// really awkward copy and reinsert for now
		updated := repo.metrics[name]
		updated.latest = Metric{toMetric(result.Latency)}
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
		"Amazon":    "amazon.com",
		"Facebook":  "facebook.com",
		"Google":    "google.com",
		"Instagram": "instagram.com",
		"LinkedIn":  "linkedin.com",
		"Netflix":   "netflix.com",
		"Reddit":    "reddit.com",
		"Twitch":    "twitch.tv",
		"Twitter":   "twitter.com",
		"Wikipedia": "wikipedia.org",
		"Yahoo":     "yahoo.com",
		"YouTube":   "youtube.com",
	}
	out := make(chan poller.Result)

	go collectSeries(out)
	for name, url := range endpoints {
		go poller.Poll(
			Endpoint{strings.ToLower(name),
				"https://www." + url},
			out,
			1*time.Second)
	}

	http.HandleFunc("/metrics", dumpMetrics)
	log.Fatal(http.ListenAndServe(":8081", nil))
}
