#![allow(unused_imports)]
use std::{
    collections::HashMap,
    hash::Hash,
    io::{Read, Write},
    net::TcpListener,
    sync::{Arc, Mutex},
    usize,
};

const OK: &str = "+OK\r\n";
const PONG: &str = "+PONG\r\n";
const NULL_BULK: &str = "$-1\r\n";

fn str_to_response(message: &str) -> String {
    return format!("${}\r\n{}\r\n", message.len(), message);
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let in_memory_map: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let in_memory_map_instance = in_memory_map.clone();
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
                            "ping" => {
                                response = PONG.to_string();
                            }
                            "get" => match args.get(1) {
                                Some(map_key) => {
                                    let in_memroy_map_lock = in_memory_map_instance.lock().unwrap();

                                    response = match in_memroy_map_lock.get(*map_key) {
                                        Some(value) => str_to_response(value),
                                        None => NULL_BULK.to_string(),
                                    };

                                    drop(in_memroy_map_lock);
                                }
                                None => {
                                    println!("error");
                                    break;
                                }
                            },
                            "set" => {
                                let mapkey = args.get(1);
                                let mapvalue = args.get(2);
                                if mapkey.is_some() && mapvalue.is_some() {
                                    let mapkey = mapkey.unwrap();
                                    let mapvalue = mapvalue.unwrap();

                                    let mut in_memroy_map_lock = in_memory_map_instance.lock().unwrap();

                                    in_memroy_map_lock.insert(mapkey.to_string(), mapvalue.to_string());

                                    response = OK.to_string();
                                    drop(in_memroy_map_lock);
                                } else {
                                    println!("error");
                                    break;
                                }
                            }
                            "info" | "quit" => response = OK.to_string(),
                            "echo" => match args.get(1) {
                                Some(echo_message) => response = str_to_response(echo_message),
                                None => {
                                    println!("error");
                                    break;
                                }
                            },
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
