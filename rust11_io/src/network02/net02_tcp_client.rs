use std::net::TcpStream;
use std::io::{Read, Write};

pub fn tcp_client() {
    let mut stream = TcpStream::connect("127.0.0.1:8888")
        .expect("server connect failed");

    let request = "request client message";

    stream.write(request.as_bytes()).unwrap();

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("[client] : {}", String::from_utf8_lossy(&buffer));
}
