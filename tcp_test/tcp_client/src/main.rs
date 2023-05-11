use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let addr = "localhost:5000";
    let mut stream = TcpStream::connect(addr).unwrap();
    stream.write("Hello World".as_bytes()).unwrap();

    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();

    println!(
        "Response from server:{:?}",
        str::from_utf8(&buffer).unwrap()
    );
}
