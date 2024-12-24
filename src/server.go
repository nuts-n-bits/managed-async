// server.go
package main

import (
    "fmt"
    "net"
    "os"
    "io"
    "bufio"
    "strconv"
)

func main() {
    listener, err := net.Listen("tcp", ":9098")
    if err != nil {
        fmt.Println("Error starting server:", err)
        os.Exit(1)
    }
    defer listener.Close()

    for {
        conn, err := listener.Accept()
        if err != nil {
            fmt.Println("Error accepting connection:", err)
            continue
        }
        go handleConnection(conn)
    }
}

func computeAverage(buf []byte) float64 {
    sum := 0.0
    is_div := true;
    for i := 0; i < len(buf); i += 1 {
        v := float64(buf[i])
        if is_div {
            v /= 3.14
        } else {
            v *= 2.71
        }
        is_div = !is_div
        sum += v
    }
    return sum / 1024;
}

func handleConnection(conn net.Conn) {
    defer conn.Close()

	buf := make([]byte, 1024)
    for true {
        _, err := io.ReadFull(bufio.NewReader(conn), buf)
        if err != nil {
            return
        }
        avg := computeAverage(buf);

        _, err = conn.Write([]byte(strconv.FormatFloat(avg, 'f', -1, 64) + "\n"))
        if err != nil {
            return
        }
    }
}
