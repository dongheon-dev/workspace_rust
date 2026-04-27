use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn read_message(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("[server] : {}", String::from_utf8_lossy(&buffer));

    let response = "response server message\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn tcp_server() {
    let listener = TcpListener::bind("127.0.0.1:8888").expect("server binding failed");

    if let Ok((stream, addr)) = listener.accept() {
        println!("[server] : tcp port {:?}", addr);
        read_message(stream);
    }
}
