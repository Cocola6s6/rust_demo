use super::router::router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self{
        Server { socket_addr, }
    }

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on: {}", self.socket_addr);

        for stream in connection_listener.incoming() {
            // 监听建立连接
            let mut stream = stream.unwrap();
            println!("Connection established");

            // 将信息存入buffer
            let mut read_buffer = [0; 254];
            stream.read(&mut read_buffer).unwrap(); // TOOD 为什么用可变引用, 因为read_buffer是可变的
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();   // TOOD 为什么能转化为HttpRequest
            Router::route(req, &mut stream);
        }
    }
}
