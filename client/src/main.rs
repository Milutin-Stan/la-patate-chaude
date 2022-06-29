use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:7878") {
        Ok(mut stream) => {
            println!("Connected to server in port 7878");

            //let text = "{\"Subscribe\":{\"name\":\"free_patato\"}}";
            let text = "\"Hello\"";
            let json = serde_json::to_string(&text).unwrap();

            println!("Send Request: {}", json.to_string());

            stream.write(&(text.len() as u32).to_be_bytes()).unwrap();
            stream.write(text.as_bytes()).unwrap();

            println!("Sent Hello, awaiting reply...");

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
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    println!("Terminated.");
}
