use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(move || handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let message = String::from("Message\n");
    stream.write(message.as_bytes()).unwrap();
}
