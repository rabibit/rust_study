use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let f = File::open("hello.txt");
    println!("Hello, world!");
    read_username_from_file().expect("something wrong");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}