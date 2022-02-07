use std::net::TcpStream;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:9000").unwrap();

    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();

    reader.read_line(&mut buffer).unwrap();
    print!("{}", buffer);
}
