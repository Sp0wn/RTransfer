use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::io::prelude::*;
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").unwrap();
    println!("Server started at {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(move || handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("Connection from: {}", stream.local_addr().unwrap());

    let mut file = File::open("Server.rs").unwrap();
    let mut buffer = [0u8; 4096];

    loop {
        let nbytes = file.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
        if nbytes < buffer.len() { break; }
    }
}
