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
                println!("Request {}", s);
                // break connection if message is invalid
                if s == "n" {
                    stream.write_all(b"malformed").unwrap();
                    stream.flush().unwrap();
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
                    stream.write_all(b"malformed").unwrap();
                    stream.flush().unwrap();
                    break; 
                }

                let prime = is_prime(req.number);
                let resp = Response {
                    method: "isPrime".to_string(),
                    prime: prime,
                };
                let mut resp_bytes = serde_json::to_vec(&resp).unwrap();
                resp_bytes.push(b'\n');
                stream.write_all(&resp_bytes).unwrap();
                stream.flush().unwrap();
                 
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
