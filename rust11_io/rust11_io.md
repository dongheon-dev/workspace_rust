# Rust11_IO_Network



### 01. IO

- OpenOptions : 없으면 생성(create), 있으면 추가(append)
- writeln! : macro



*io/io01_console.rs*

```rust
use std::io;

pub fn console_io() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    print!("{input}");
}

```

<br>

*io/io02_file.rs*

```rust
use std::fs::{File, read_to_string};
use std::io::{Result, Write, Read, BufReader, BufRead};

pub fn file_write() -> Result<()> {
    let mut file = File::create("rust.txt")?;
    file.write("hello\n".as_bytes())?;
    file.write(b"world\n")?;

    Ok(())
}


pub fn file_append() -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("rust.txt")?;

    writeln!(file, "hello, rust!\n")?;

    Ok(())
}

pub fn file_read01() -> Result<()> {
    let content = read_to_string("rust.txt")?;
    println!("{content}");

    Ok(())
}

pub fn file_read02() -> Result<()> {
    let mut file = File::open("rust.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    println!("{content}");
    Ok(())
}

// Buffer 사용
pub fn file_read03() -> Result<()> {
    let file = File::open("rust.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

```

<br>

*io.rs*

```rust
pub mod io01_console;
pub mod io02_file;

```



*main.rs*

```rust
mod io;

use io::io02_file::*;

fn main() {
    io::io01_console::console_io();
    file_write().expect("file write error");
	file_append().expect("file append error");
    file_read01().expect("file read error");
    file_read02().expect("file read error");
    file_read03().expect("file read error");
}

```

<br>



### 02. network 

#### 2-1. network01

- minreq : 경량 client (동기 전용)
- reqwest : 기능이 많고, 비동기 처리 가능 (blocking : 동기 / async : 비동기) -> 내부적으로 hyper 사용
- hyper : tokio 기반 저수준 http library -> web service 만들 때 공부

**Cargo.toml**

```toml
[package]
name = "rust11_io"
version = "0.1.0"
edition = "2024"

[dependencies]
minreq = {version = "2.14.1", features = ["https"]}
reqwest = {version = "0.13.2", features = ["blocking"]}

```

<br>

*network01/net01_minreq.rs*

```rust
pub fn minreq_get() {
    let response = minreq::get("https://dongheon-dev.github.io/reviewlog/robots.txt")
        .send()
        .expect("request failed");

    println!("status: {}", response.status_code);

    let body = response.as_str().unwrap();
    println!("{body}");
}

```

<br>

*network01/net02_reqwest.rs*

```rust
use reqwest::blocking::Client;

pub fn reqwest_get() {
    let url = "https://dongheon-dev.github.io/reviewlog/robots.txt";
    let client = Client::new();

    let response = client.get(url)
        .header("User-Agent", "Rust-App")
        .send()
        .expect("[reqwest] : 요청 실패");

    println!("[reqwest] : status {}", response.status());

    let body = response.text().expect("[reqwest] : 본문 읽기 실패");
    println!("[reqwest] : body\n{}", body);
}

```

<br>

*network01.rs*

```rust
pub mod net01_minreq;
pub mod net02_reqwest;

```

<br>

*main.rs*

```rust
mod io;
mod network01;

use io::io02_file::*;

fn main() {
    io::io01_console::console_io();
    file_write().expect("file write error");
    file_read01().expect("file read error");
    file_read02().expect("file read error");
    file_read03().expect("file read error");

    network01::net01_minreq::minreq_get();
    network01::net02_reqwest::reqwest_get();
}

```

<br>



#### 2-2. network02

- tcp (Transmission Control Protocol) : **3 way handshake** -> byte stream
- udp (User Datagram Protocol) : connectionless -> packet

*network02/net01_tcp_server.rs*

```rust
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

```

<br>

*network02/net02_tcp_client.rs*

```rust
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

```

<br>

*network03/net03_udp_server.rs*

```rust
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

```

<br>

*network04/net04_udp_client.rs*

```rust
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

```

<br>

**lib.rs**

```rust
pub mod network02;

```



**bin/**

*tcp_server.rs*

```rust
use rust11_io::network02::net01_tcp_server;

fn main() {
    net01_tcp_server::tcp_server();
}

```

<br>

*tcp_client.rs*

```rust
use rust11_io::network02::net02_tcp_client;

fn main() {
    net02_tcp_client::tcp_client();
}

```

<br>

*udp_server.rs*

```rust
use rust11_io::network02::net03_udp_server;

fn main() {
    net03_udp_server::udp_server();
}

```

<br>*udp_client.rs*

```rust
use rust11_io::network02::net04_udp_client;

fn main() {
    net04_udp_client::udp_client();
}

```

<br>

**Cargo.toml**

```toml
[package]
name = "rust11_io"
version = "0.1.0"
edition = "2024"

default-run = "rust11_io"

[dependencies]
minreq = {version = "2.14.1", features = ["https"]}
reqwest = {version = "0.13.2", features = ["blocking"]}

```

<br>

**실행방법**

```bash
# tcp
cargo run --bin tcp_server
# 새 터미널에서
cargo run --bin tcp_client

# udp
cargo run --bin udp_server
# 새 터미널에서
cargo run --bin udp_client

# cargo.toml 에서 default-run 을 지정해주지 않으면 cargo run 했을때 어떤 binary 를 실행할건지 물어보게 된다!!
```

