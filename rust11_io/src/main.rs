mod io;
mod network01;

use io::io02_file::*;

fn main() {
    io::io01_console::console_io();
    file_write().expect("file write error");
    file_append().expect("file append error");
    file_read01().expect("file read error");
    file_read02().expect("file read error");
    file_read03().expect("file read error");

    network01::net01_minreq::minreq_get();
    network01::net02_reqwest::reqwest_get();
}
