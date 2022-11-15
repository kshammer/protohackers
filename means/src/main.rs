use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use std::collections::BTreeMap; 
use std::convert::TryInto;
use std::ops::Bound::Included;

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
    let mut map:BTreeMap<i32, i32>  = BTreeMap::new(); 
    loop {
        let mut read = [0; 9];
        stream.read_exact(&mut read).unwrap();
        let operation = read[0];
        println!("Operation {}", operation);
        if operation == 73 {
            let time = i32::from_be_bytes(read[1..5].try_into().unwrap());
            let value = i32::from_be_bytes(read[5..9].try_into().unwrap());
            map.insert(time,value);
            println!("Inserted time {}, {}", time, value);
        } else {
            let min = i32::from_be_bytes(read[1..5].try_into().unwrap());
            let max = i32::from_be_bytes(read[5..9].try_into().unwrap());
             if min > max {
                stream.write(&0_i32.to_be_bytes()).unwrap();
                continue;
            }
            let mut prices = Vec::new(); 
            for (&_key, &value) in map.range((Included(&min), Included(&max))) {
                prices.push(value);
            }
            if prices.is_empty(){
                stream.write(&0_i32.to_be_bytes()).unwrap();
                continue;
            }
            let prices_sum = prices.iter().fold(0, |a, &b| a+b as i64);
            let avg = prices_sum / prices.len() as i64; 
            let avg = avg as i32;
            println!("Average {}", avg);
            stream.write(&avg.to_be_bytes()).unwrap();

        }
    }
}
