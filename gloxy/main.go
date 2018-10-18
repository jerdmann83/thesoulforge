package main

import (
	"bytes"
	"flag"
	"fmt"
	"log"
	"net"
	"os"
	"time"
)

func onServerConn(conn net.Conn) {
	log.Printf("server: handle %v\n", conn)
	time.Sleep(1 * time.Hour)
}

func onClientConn(conn net.Conn) {
	conn.SetDeadline(time.Now().Add(1 * time.Second))
	buf := bytes.Repeat([]byte("zx"), 512)
	count, err := conn.Write(buf)
	if err != nil {
		log.Printf("onClientConn: %v", err)
	} else {
		log.Printf("onClientConn: wrote %v bytes", count)
	}

}

func main() {
	hostname, _ := os.Hostname()
	logger := log.New(os.Stdout, hostname+" ", log.LstdFlags)

	var port uint
	var server string
	var connections uint
	flag.UintVar(&port, "port", 8080, "port")
	flag.StringVar(&server, "server", "", "server")
	flag.UintVar(&connections, "count", 1, "connections")
	flag.Parse()

	sport := fmt.Sprintf(":%v", port)
	if len(server) == 0 {
		log.Printf("serving on %v", sport)
		ln, err := net.Listen("tcp", sport)
		if err != nil {
			logger.Fatal(err)
		}

		for {
			conn, err := ln.Accept()
			if err != nil {
				logger.Fatal(err)
			}
			go onServerConn(conn)
			time.Sleep(2 * time.Second)
		}
	} else {
		dest := fmt.Sprintf("%v%v", server, sport)
		for i := uint(0); i < connections; i++ {
			log.Printf("%v connections remaining to %v", connections-i, dest)
			conn, err := net.Dial("tcp", dest)
			if err != nil {
				logger.Fatal(err)
			}
			go onClientConn(conn)
		}
		time.Sleep(1 * time.Hour)
	}

}
