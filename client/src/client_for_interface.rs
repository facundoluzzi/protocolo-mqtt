use std::io::stdin;
use std::io::BufRead;
use std::io::BufReader;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

pub struct Client {}

impl Client {
    pub fn connect_to_server(host: String, port: String) {
        let reader = BufReader::new(stdin());
        let addres = format!("{}:{}", host, port);
        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                println!("Successfully connected to server in port 3333");
                for line in reader.lines().flatten() {
                    if let Err(msg_error) = stream.write(line.as_bytes()) {
                        println!("{}", msg_error);
                        return;
                    }
                    println!("Sent {}, awaiting reply...", line);

                    let mut data = vec![0_u8; 100]; // using 6 byte buffer
                    match stream.read(&mut data) {
                        Ok(size) => {
                            println!("received {:?}", from_utf8(&data[0..size]));
                        }
                        Err(e) => {
                            println!("Failed to receive data: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
        println!("Terminated.");
    }
}
