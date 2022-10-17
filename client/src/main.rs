use std::net::TcpStream;
use std::io::Write;
use std::io::Read;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3333").unwrap();
    let cool_str = "{\"key\": \"value\"}"; 
    stream.write(cool_str.as_bytes()).unwrap();
}
