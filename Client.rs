use std::net::TcpStream;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9000").unwrap();
    
    let mut file = File::create("prueba.txt").unwrap();
    let mut buffer = [0u8; 4096];

    loop {
        let check = stream.read(&mut buffer).unwrap();
        if check == 0 { break; }
        file.write(&mut buffer).unwrap();
    } 
}
