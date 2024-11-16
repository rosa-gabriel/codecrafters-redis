#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                std::thread::spawn(move || {
                    let mut buf = [0; 512];
                    loop {
                        let count = stream.read(&mut buf).unwrap();

                        if count == 0 {
                            break;
                        }

                        let incoming_message = String::from_utf8_lossy(&buf);
                        println!("{}", incoming_message);

                        stream.write(b"+PONG\r\n").unwrap();
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
