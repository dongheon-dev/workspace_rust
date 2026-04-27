use std::net::UdpSocket;

fn read_message(socket: UdpSocket) {
    let mut buffer = [0; 1024];

    let (size, addr) = socket.recv_from(&mut buffer).expect("server socket failed");

    println!("[server] : {}", String::from_utf8_lossy(&buffer[..size]));

    let response = "response server message\n";

    socket.send_to(response.as_bytes(), addr).expect("server response failed");
}

pub fn udp_server() {
    let socket = UdpSocket::bind("127.0.0.1:9999").expect("server binding failed");
    println!("[server] : udp port 127.0.0.1:9999");

    read_message(socket);
}
