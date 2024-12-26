// server.go
package main

import (
    "fmt"
    "net"
    "os"
    "io"
    "bufio"
    "strconv"
    "runtime"
)

type ProcessRequest struct {
    buf *[]byte
    resChan chan float64
}

func main() {
    runtime.GOMAXPROCS(1)
    listener, err := net.Listen("tcp", ":9098")
    if err != nil {
        fmt.Println("Error starting server:", err)
        os.Exit(1)
    }
    defer listener.Close()

    requestQueue := make(chan ProcessRequest);

    go computeAverage(requestQueue);

    for {
        conn, err := listener.Accept()
        if err != nil {
            fmt.Println("Error accepting connection:", err)
            continue
        }
        go handleConnection(conn, requestQueue)
    }
}

func computeAverage(requestQueue chan ProcessRequest) float64 {
    for {
        request := <-requestQueue
        sum := 0.0
        is_div := true;
        for i := 0; i < len(*request.buf); i += 1 {
            v := float64((*request.buf)[i])
            if is_div {
                v /= 3.14
            } else {
                v *= 2.71
            }
            is_div = !is_div
            sum += v
        }
        request.resChan <- sum / 1024;
    }
}

func handleConnection(conn net.Conn, requestQueue chan ProcessRequest) {
    defer conn.Close()

	buf := make([]byte, 1024)
    var request ProcessRequest;
    request.buf = &buf;
    request.resChan = make(chan float64);
    for true {
        _, err := io.ReadFull(bufio.NewReader(conn), buf)
        if err != nil {
            return
        }
        requestQueue <- request;
        avg := <-request.resChan;

        _, err = conn.Write([]byte(strconv.FormatFloat(avg, 'f', -1, 64) + "\n"))
        if err != nil {
            return
        }
    }
}
