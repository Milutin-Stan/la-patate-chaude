use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:7878") {
        Ok(mut stream) => {
            println!("Connected to server in port 7878");

            let text = "{\"Subscribe\":{\"name\":\"free_patato\"}}";
            let json = serde_json::to_string(&text).unwrap();

            println!("Send Request: {}", json.to_string());

            stream.write(&(text.len() as u32).to_be_bytes()).unwrap();
            stream.write(text.as_bytes()).unwrap();

            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 4];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    println!(from_utf8(&data) as u32)
                    let text = from_utf8(&data).unwrap();
                    println!("reply: {}", text);

                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    println!("Terminated.");
}
