use std::fs::{File, OpenOptions, read_to_string};
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
