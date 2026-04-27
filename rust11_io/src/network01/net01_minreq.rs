pub fn minreq_get() {
    let response = minreq::get("https://dongheon-dev.github.io/reviewlog/robots.txt")
        .send()
        .expect("request failed");

    println!("status: {}", response.status_code);

    let body = response.as_str().unwrap();
    println!("{body}");
}
