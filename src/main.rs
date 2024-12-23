#![allow(dead_code)]

use std::{
    fs::File,
    io::{self, Write},
    os::unix::io::FromRawFd,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let argv = std::env::args().collect::<Vec<String>>();
    let first_arg = argv.get(1).map(|s| s.as_str());
    match first_arg {
        Some("client") => run_client().await,
        _ => run_server().await,
    }
}

async fn run_server() {
    // Bind the listener to the address
    let listener = tokio::net::TcpListener::bind("::0:9098").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(mut socket: tokio::net::TcpStream) {
    let addr = socket.peer_addr().unwrap();
    println!("connected to {:?}", addr);
    let mut buf = [0u8; 1024];
    loop {
        buf = [0u8; 1024];
        let read = socket.read_exact(&mut buf[..]).await;
        dbg!(&read);
        dbg!(std::str::from_utf8(&buf[..]));

        if read.unwrap_or(0) == 0 {
            socket.flush();
            std::mem::drop(socket);
            println!("socket {:?} will be dropped", addr);
            break;
        }

        let returnable = average(work(&buf)).to_string();


        let write = socket.write_all(returnable.as_bytes()).await;
        match write {
            Err(_) => {
                break;
            }
            Ok(_) => {}
        }


    }
}

async fn run_client() {
    let mut stream = tokio::net::TcpStream::connect("127.0.0.1:9098")
        .await
        .unwrap();
    let write = stream.write("afrwahgreggreshtrshtrhtrdhtrdhtrhthtresdgtrhtrdhtrsh".as_bytes());
    dbg!(write);
}

const PI: f64 = 3.14;
const E: f64 = 2.71;

fn work(incoming_bytes: &[u8; 1024]) -> [f64; 1024] {
    let mut result = [0f64; 1024];
    for (i, elem) in incoming_bytes.iter().enumerate() {
        if i & 0b1 == 0 {
            result[i] = (*elem as f64) * PI;
        } else {
            result[i] = (*elem as f64) / E;
        }
    }
    result
}

fn average(ns: [f64; 1024]) -> f64 {
    let mut sum = 0f64;
    for i in ns {
        sum += i 
    }
    return sum / 1024f64;
}