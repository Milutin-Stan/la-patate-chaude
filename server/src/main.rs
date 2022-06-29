use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use serde_json::{Result, Value};
use std::str;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let _test = handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Value JSON {}", String::from_utf8_lossy(&buffer[..]));

    stream.read(&mut buffer).unwrap();

    println!("Value JSON {}", String::from_utf8_lossy(&buffer[..]));

}

//let t = str::from_utf8(&buffer[137..153]).unwrap();
//println!("t from buffer {}", t);
//let data = r#"{"test": "test"}"#;
//println!("data from local {}", data);
//let v: Value = serde_json::from_str(t)?;

//println!("Test: {}", String::from_utf8_lossy(&buffer[135..]));
