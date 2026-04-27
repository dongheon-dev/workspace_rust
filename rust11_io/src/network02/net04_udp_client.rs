use std::net::UdpSocket;

pub fn udp_client() {
    let socket = UdpSocket::bind("127.0.0.1:0").expect("udp client bind error");
    let server_addr = "127.0.0.1:9999";
    let request = "request client udp message";

    socket.send_to(request.as_bytes(), server_addr).expect("client request failed");

    let mut buffer = [0; 1024];
    let (size, _) = socket.recv_from(&mut buffer).expect("client response failed");

    println!("[client] : {}", String::from_utf8_lossy(&buffer[..size]));
}
