use std::{
    io::{Read, Write},
    net::TcpStream,
    thread,
};

use crate::packet_manager::PacketManager;
use std::sync::mpsc;

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

    fn send_connect(&self, user: String, password: String, stream: &TcpStream, id_client: String) {
        match Option::Some(stream) {
            Some(mut stream) => {
                let mut flags: u8 = 0x00;
                let mut bytes = vec![
                    0x10, //Packet ID
                    //0x00, Remaining Length
                    0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, // Variable Header
                    0x04, // Protocol
                    0x00, //Flags
                    0x00, 0x0B, //Keep Alive
                ];
                add_client_id_bytes(id_client, &mut bytes);
                add_username_bytes(user, &mut flags, &mut bytes);
                add_password_bytes(password, &mut flags, &mut bytes);
                bytes[8] = flags;
                let length = bytes.len();
                bytes.insert(1, (length - 1) as u8);
                stream.write_all(&bytes).unwrap();
            }
            None => panic!("No pude enviar"),
        }
    }

    fn check_connack_code(&self, code: u8) -> Result<String, String> {
        match code {
            0x00 => Ok("Conexion realizada con exito".to_string()),
            0x01 => Err("Error: la version del protocolo no es compatible".to_string()),
            0x04 => Err("Error: los datos enviados no son correctos".to_string()),
            0x05 => Err("Error: no esta autorizado".to_string()),
            _ => Err("Error desconocido".to_string()),
        }
    }

    pub fn connect_to_server(
        mut self,
        host: String,
        port: String,
        user: String,
        password: String,
        id_client: String,
    ) -> Result<String, String> {
        let address = format!("{}:{}", host, port);
        match TcpStream::connect(address) {
            Ok(stream) => {
                let (tx, rx) = mpsc::channel::<u8>();
                let stream_copy = stream.try_clone().expect("Could not clone");
                self.send_connect(user, password, &stream_copy, id_client);
                thread::spawn(move || {
                    receive_responses_from_broker(stream_copy, tx);
                });
                let connack_code_received = rx.recv().unwrap();
                self.stream = Some(stream);
                self.check_connack_code(connack_code_received)
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
                Err("La conexion no se ha podido establecer".to_string())
            }
        }
    }
}

fn receive_responses_from_broker(mut stream: TcpStream, channel_producer: mpsc::Sender<u8>) {
    let mut data = vec![0_u8; 100]; // using 6 byte buffer
    while match stream.read(&mut data) {
        Ok(_) => {
            let packet_manager = PacketManager::new();
            let packet_received = packet_manager.process_message(&data);
            println!("received {:?}", &packet_received.get_type());
            let code = packet_received.get_status_code();
            channel_producer.send(code).unwrap();
            true
        }
        Err(e) => {
            println!("Failed to receive data: {}", e);
            false
        }
    } {}
}

fn add_client_id_bytes(id_client: String, bytes: &mut Vec<u8>) {
    if !id_client.is_empty() {
        let id_length = id_client.len();
        let mut id_client_in_bytes = id_client.as_bytes().to_vec();
        bytes.push(0x00);
        bytes.push(id_length as u8);
        bytes.append(&mut id_client_in_bytes);
    } else {
        bytes.append(&mut vec![0x00, 0x02, 0x00, 0x00]);
    }
}

fn add_password_bytes(password: String, flags: &mut u8, bytes: &mut Vec<u8>) {
    if !password.is_empty() {
        *flags |= 0b01000000;
        let password_length = password.len();
        let mut password_in_bytes = password.as_bytes().to_vec();
        bytes.push(0x00);
        bytes.push(password_length as u8);
        bytes.append(&mut password_in_bytes);
    }
}

fn add_username_bytes(user: String, flags: &mut u8, bytes: &mut Vec<u8>) {
    if !user.is_empty() {
        *flags |= 0b10000000;
        let user_length = user.len();
        let mut user_in_bytes = user.as_bytes().to_vec();
        bytes.push(0x00);
        bytes.push(user_length as u8);
        bytes.append(&mut user_in_bytes);
    }
}
