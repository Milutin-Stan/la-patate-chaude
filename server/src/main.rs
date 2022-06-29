use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use serde_json::{Result, Value};
use std::str;
use std::str::from_utf8;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let _test = handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut data = [0 as u8; 4];
    match stream.read_exact(&mut data) {
        Ok(_) => {

            let first: u32 = u32::from_be_bytes(data.try_into().unwrap());
            println!("First: {}", first);
            let mut body = vec![0; first as usize];
            match stream.read_exact(&mut body) {
                Ok(_) => {
                    let response = from_utf8(&body).unwrap();
                    println!("response: {}", response);
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }

        },
        Err(e) => {
            println!("Failed to receive data: {}", e);
        }
    }
}

//let t = str::from_utf8(&buffer[137..153]).unwrap();
//println!("t from buffer {}", t);
//let data = r#"{"test": "test"}"#;
//println!("data from local {}", data);
//let v: Value = serde_json::from_str(t)?;

//println!("Test: {}", String::from_utf8_lossy(&buffer[135..]));
