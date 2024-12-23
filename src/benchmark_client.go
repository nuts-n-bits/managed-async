// client.go
package main

import (
    "fmt"
    "net"
    "os"
	"log"
	"io"
	"strconv"
	"time"
	"bufio"
	"math/rand"
)

func main() {
    
    if len(os.Args) != 3 {
		log.Fatal("Usage: benchmark_client <num_threads> <num_times>")
	}
	num_threads, err := strconv.Atoi(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	num_times, err := strconv.Atoi(os.Args[2])
	if err != nil {
		log.Fatal(err)
	}
	// data, err := os.ReadFile(os.Args[3])
	// if err != nil  {
	// 	log.Fatal(err)
	// }
	data := make([]byte, 1024)
	rand.Read(data)
	c := make(chan int)
    for i := 0; i < num_threads; i++ {
		go createConnection(data, num_times, c)
	}
    total_latency := 0
	for i := 0; i < num_threads; i++ {
        total_latency += <-c
	}
	fmt.Println("Number of Connections:", num_threads)
	fmt.Println("Size:", len(data))
	fmt.Println("Times Sent Per Connection:", num_times)
	fmt.Println("Average Latency (ns):", total_latency / (num_threads * num_times))
	
}

func createConnection(data []byte, num_times int, c chan int) {
    conn, err := net.Dial("tcp", "127.0.0.1:9098")
    if err != nil {
        log.Fatal(err)
    }
    defer conn.Close()
	echo_data := make([]byte, len(data))
	reader := bufio.NewReader(conn)
	cumulative_latency := 0
	for i := 0; i < num_times; i++ {
        s := time.Now()
        _, err = conn.Write(data)
		if err != nil  {
			log.Fatal(err)
		}
        _, err := io.ReadFull(reader, echo_data)
		if err != nil {
			log.Fatal(err)
		}
		cumulative_latency += int(time.Since(s))
		time.Sleep(1 * time.Second)
	}
    c <- cumulative_latency
}