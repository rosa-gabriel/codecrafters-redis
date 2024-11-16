#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::TcpListener,
    usize,
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
                        let message_parts: Vec<&str> = incoming_message.split("\r\n").collect();

                        if message_parts.len() < 1 {
                            break;
                        }

                        let arg_count_str = message_parts.get(0).unwrap();

                        if !arg_count_str.contains("*") {
                            break;
                        }

                        let arg_count: usize =
                            arg_count_str.replace("*", "").parse::<usize>().unwrap();

                        let mut args: Vec<&str> = Vec::new();

                        if arg_count > 0 {
                            let mut curr_index: usize = 2;
                            while curr_index != (arg_count + 1) * 2 {
                                if message_parts.len() > curr_index {
                                    args.push(message_parts.get(curr_index).unwrap());
                                    curr_index += 2;
                                } else {
                                    println!("Error");
                                    break;
                                }
                            }
                        }

                        if args.len() == 0 {
                            break;
                        }

                        let command = args.get(0).unwrap();

                        let response;

                        match command.to_lowercase().as_str() {
                            "ping" | "info" | "quit" => {
                                response = "+PONG\r\n".to_string();
                            }
                            "echo" => {
                                match args.get(1) {
                                    Some(echo_message) => {
                                        response = format!("${}\r\n{}\r\n", echo_message.len(), echo_message)
                                    },
                                    None => break,
                                }
                            }
                            _ => {
                                println!("error");
                                break;
                            }
                        };

                        stream.write(response.as_bytes()).unwrap();
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
