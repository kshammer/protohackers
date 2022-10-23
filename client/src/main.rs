use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3333").unwrap();
    let cool_str = "{\"method\":\"isPrime\",\"number\":2}";
    stream.write(cool_str.as_bytes()).unwrap();
    let mut read = [0; 1028];
    match stream.read(&mut read) {
        Ok(n) => {
            let s = match str::from_utf8(&read) {
                Ok(v) => v,
                Err(e) => panic!("Invalid {}", e),
            };
            println!("{}", s);
        }
        Err(_) => {
            panic!("memes");
        }
    }
    stream.write(cool_str.as_bytes()).unwrap();
}
