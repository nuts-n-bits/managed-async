#![allow(dead_code)]

mod langbench_sort;
mod langbench_log_parser;
mod langbench_sudoku;

use std::{
    fs::File,
    io::{self, Write},
    os::unix::io::FromRawFd,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

//#[tokio::main(flavor = "current_thread")]
#[tokio::main]
async fn main() {
    let argv = std::env::args().collect::<Vec<String>>();
    let first_arg = argv.get(1).map(|s| s.as_str());
    match first_arg {
        Some("client") => run_client().await,
        Some("sort") => langbench_sort::langbench_sort::main(),
        Some("sort-o") => langbench_sort::langbench_sort_opti::main(),
        Some("sudoku") => langbench_sudoku::sudoku::main().await,
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
        //dbg!(&read);
        //dbg!(std::str::from_utf8(&buf[..]));

        if read.unwrap_or(0) == 0 {
            socket.flush();
            std::mem::drop(socket);
            println!("socket {:?} will be dropped", addr);
            break;
        }

        let returnable = compute_average(&buf).to_string() + "\n";


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
    //dbg!(write);
}

const PI: f64 = 3.14;
const E: f64 = 2.71;

fn compute_average(buf: &[u8; 1024]) -> f64 {
    let mut sum = 0f64;
    let mut is_div = true;
    for i in 0..buf.len() {
        let mut v = buf[i] as f64;
        if is_div { 
            v /= PI 
        }
        else { 
            v *= E 
        }
        is_div = !is_div;
        sum += v;
    }
    return sum / 1024f64;
}

















