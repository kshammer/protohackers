use primes::is_prime;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
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
        let mut read = [0; 999999];
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
                    println!("Invalid Request {}", s); 
                    stream.write(b"malformed").unwrap();
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
                        stream.write(b"malformed").unwrap();
                        stream.shutdown(Shutdown::Both).unwrap();
                    }

                    let prime = prime(req.number);
                    let resp = Response {
                        method: "isPrime".to_string(),
                        prime: prime,
                    };
                    let mut resp_bytes = serde_json::to_vec(&resp).unwrap();
                    resp_bytes.push(b'\n');
                   // print!(
                    //     "Request {}, Response {}",
                    //     serde_json::to_string(&req).unwrap(),
                    //     String::from_utf8_lossy(&resp_bytes)
                    // );
                    stream.write(&resp_bytes).unwrap();
                }
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}

fn prime(num: f64) -> bool {
    if num <= 1.0 {
        return false;
    }
    if num.fract() != 0.0 {
        return false;
    }
    is_prime(num as u64)
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
