use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

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
                if n == 0 {
                    break;
                }
                let s = match str::from_utf8(&read) {
                    Ok(v) => v.trim_matches(char::from(0)),
                    Err(_) => "n",
                };
                println!("{}", s);
                // break connection if message is invalid
                if s == "n" {
                    stream.write(&read[0..n]).unwrap();
                    break;
                }
                let req: Request = match serde_json::from_str(&s) {
                    Ok(r) => r,
                    Err(_) => Request {
                        method: "notreal".to_string(),
                        number: 0,
                    },
                };
                // break connection if method is not isPrime
                if req.method != "isPrime" {
                    stream.write(&read[0..n]).unwrap();
                    break;
                }

                let prime = is_prime(req.number);
                let resp = Response {
                    method: "isPrime".to_string(),
                    prime: prime,
                };
                let resp_serial = serde_json::to_string(&resp).unwrap();
                stream.write(resp_serial.as_bytes()).unwrap();
                
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

#[derive(Serialize, Deserialize)]
struct Request {
    method: String,
    number: i64,
}

#[derive(Serialize, Deserialize)]
struct Response {
    method: String,
    prime: bool,
}
