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
