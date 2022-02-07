use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9000").unwrap();
}
