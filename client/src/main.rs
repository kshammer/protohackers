use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3333").unwrap();
    let cool_str = "{\"method\":\"isPrime\",\"number\":2}\n";
    stream.write(cool_str.as_bytes()).unwrap();
}
