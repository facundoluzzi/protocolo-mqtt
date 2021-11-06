use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::Sender;

pub struct Default {}

impl Default {
    pub fn init(_bytes: &[u8]) -> Default {
        Default {}
    }

    pub fn get_type(&self) -> String {
        "default".to_owned()
    }

    pub fn send_response(&self, mut stream: &TcpStream) {
        if let Err(msg_error) = stream.write(b"default message\n") {
            println!("Error in sending response: {}", msg_error);
        }
    }

    pub fn send_message(&self, stream: &Sender<String>){
        //todo
    }
}
