package main

import (
	"fmt"
	"math/rand"
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

func churnValues(out chan Metric, interval time.Duration) {
	for {
		out <- Metric{time.Now().Unix(), rand.Float64()}
		time.Sleep(interval)
	}
}

func main() {
	out := make(chan Metric)
	go churnValues(out, 1*time.Second)

	for {
		s := <-out
		fmt.Printf("got %s\n", s)
	}
	fmt.Println("vim-go")
}
