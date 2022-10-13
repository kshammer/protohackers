use std::thread; 
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server is started");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New Connection {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    drop(listener);
}

fn handle_client(mut stream: TcpStream){
    let mut data = [0 as u8; 50];
    while match stream.read(&mut data) {
        Ok(size) => {
            stream.write(&data[0..size]).unwrap();
            true
        }
        Err(_) => {
            println!("An error occured, terminating connection {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}
