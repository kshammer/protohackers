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
                // break connection if message is invalid
                if s == "n" {
                    stream.write_all(b"malformed").unwrap();
                    stream.flush().unwrap();
                    break;
                }

                let split = s.lines();
                for s in split {
                    let req: Request = match serde_json::from_str(&s) {
                        Ok(r) => r,
                        Err(_) => Request {
                            method: "notreal".to_string(),
                            number: 0.0,
                        },
                    };
                    // break connection if method is not isPrime
                    if req.method != "isPrime" {
                        println!("Invalid Request {}, Response {}", s, "malformed");
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
                    print!(
                        "Request {}, Response {}",
                        serde_json::to_string(&req).unwrap(),
                        String::from_utf8_lossy(&resp_bytes)
                    );
                    stream.write_all(&resp_bytes).unwrap();
                    stream.flush().unwrap();
                }
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}

fn is_prime(num: f64) -> bool {
    if num <= 1.0 {
        return false;
    }
    if num.fract() != 0.0 {
        return false; 
    }
    let int = num as i64;
    for a in 2..int {
        if int % a == 0 {
            return false;
        }
    }
    true
}

#[derive(Serialize, Deserialize)]
struct Request {
    method: String,
    number: f64,
}

#[derive(Serialize, Deserialize)]
struct Response {
    method: String,
    prime: bool,
}
