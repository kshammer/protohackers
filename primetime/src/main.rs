use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;
use std::thread;
use serde_json::{Result, Value};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server is started");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New Connection {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    drop(listener);
}

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                println!("stream {:?}", read);
                if n == 0 {
                    break;
                }
                let s = match str::from_utf8(&read) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                println!("Converted {}", s); 
                let v: Value = serde_json::from_str(s).unwrap();
                println!("serde value {}", v); 
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}

fn is_prime(num: i64) -> bool {
    if num <= 1 {
        return false;
    }
    for a in 2..num {
        if num % a == 0 {
            return false;
        }
    }
    true
}
