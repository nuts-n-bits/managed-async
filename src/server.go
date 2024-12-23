// server.go
package main

import (
    "fmt"
    "net"
    "os"
    "io"
    "bufio"
    "encoding/binary"
    "math"
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

func handleConnection(conn net.Conn) {
    defer conn.Close()

	buf := make([]byte, 1024)
    for true {
        n, err := io.ReadFull(bufio.NewReader(conn), buf)
        if err != nil {
            return
        }
        is_div := true;
        for i := 0; i < len(buf); i += 8 {
            v := math.Float64frombits(binary.LittleEndian.Uint64(buf[i : i+8]))
            if is_div {
                v /= 3.14
            } else {
                v *= 2.71
            }
            is_div = !is_div
            binary.LittleEndian.PutUint64(buf[i:i+8], math.Float64bits(v))
        }

        _, err = conn.Write(buf[:n])
        if err != nil {
            return
        }
    }
}
