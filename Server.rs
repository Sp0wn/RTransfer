use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
    }
}
