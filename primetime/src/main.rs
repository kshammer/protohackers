use primes::is_prime;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write, BufReader, BufRead};
use std::net::{Shutdown, TcpListener, TcpStream};
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
    let reader = BufReader::new(stream.try_clone().unwrap());
    let mut lines = reader.lines(); 
    while let Some(Ok(line)) = lines.next(){
        let req: Request =match serde_json::from_str(&line){
            Ok(r) => r, 
            Err(_) => Request {
                method: "notread".to_string(),
                number: 0.0,
            }
        };
        if req.method != "isPrime"{
            stream.write(b"malformed").unwrap();
            stream.shutdown(Shutdown::Both).unwrap();
        }
        let prime = prime(req.number);
        let resp = Response{
            method: "isPrime".to_string(),
            prime, 
        };
        let mut resp_bytes = serde_json::to_vec(&resp).unwrap();
        resp_bytes.push(b'\n');
        stream.write(&resp_bytes).unwrap();
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
