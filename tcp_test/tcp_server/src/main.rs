use std::net::TcpListener;
use std::io::{Read, Write};


fn main() {
    let addr = "localhost:5000";
    let listener = TcpListener::bind(addr).unwrap();
    println!("Tcp server running=========================>");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
