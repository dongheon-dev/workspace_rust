use std::io;

pub fn console_io() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    print!("{input}");
}
