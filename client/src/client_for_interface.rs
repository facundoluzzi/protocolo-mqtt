use std::{
    io::{Read, Write},
    net::TcpStream,
    thread,
};

use crate::packet_factory::PacketManager;
pub struct Client {
    stream: Option<TcpStream>,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Client {
        Client { stream: None }
    }

    fn send_connect(self, user: String, password: String) {
        match self.stream {
            Some(mut stream) => {
                let mut flags: u8 = 0x00;
                let mut bytes = vec![
                    0x10, //Packet ID
                    //0x00, Remaining Length
                    0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, // Variable Header
                    0x04, // Protocol
                    0x00, //Flags
                    0x00, 0x0B, //Keep Alive
                    0x00, 0x02, 0x00, 0x00, // Payload - Client ID
                ];
                if !user.is_empty() {
                    flags |= 0b10000000;
                    let user_length = user.len();
                    let mut user_in_bytes = user.as_bytes().to_vec();
                    bytes.push(0x00);
                    bytes.push(user_length as u8);
                    bytes.append(&mut user_in_bytes);
                }
                if !password.is_empty() {
                    flags |= 0b01000000;
                    let password_length = password.len();
                    let mut password_in_bytes = password.as_bytes().to_vec();
                    bytes.push(0x00);
                    bytes.push(password_length as u8);
                    bytes.append(&mut password_in_bytes);
                }
                bytes[8] = flags;
                let length = bytes.len();

                bytes.insert(1, (length - 1) as u8);
                stream.write_all(&bytes).unwrap();
            }
            None => panic!("No pude enviar"),
        }
    }

    pub fn connect_to_server(
        mut self,
        host: String,
        port: String,
        user: String,
        password: String,
    ) -> Result<String, String> {
        let address = format!("{}:{}", host, port);
        match TcpStream::connect(address) {
            Ok(stream) => {
                let stream_clone = stream.try_clone().expect("Could not clone the stream");
                self.stream = Some(stream_clone);
                self.send_connect(user, password);
                thread::spawn(move || receive_responses_from_broker(stream));
                Ok("La conexion ha sido exitosa".to_string())
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
                Err("La conexion no se ha podido establecer".to_string())
            }
        }
    }
}

fn receive_responses_from_broker(mut stream: TcpStream) {
    let mut data = vec![0_u8; 100]; // using 6 byte buffer
    while match stream.read(&mut data) {
        Ok(_) => {
            let packet_manager = PacketManager::new();
            println!(
                "received {:?}",
                packet_manager.process_message(&data).get_type()
            );
            true
        }
        Err(e) => {
            println!("Failed to receive data: {}", e);
            false
        }
    } {}
}
