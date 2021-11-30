use std::{
    io::{Read, Write},
    net::TcpStream,
    thread,
};

use crate::packet_manager::PacketManager;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

pub struct Client {
    stream: Option<TcpStream>,
    sender: Option<Sender<(usize, Vec<String>)>>,
}

impl Clone for Client {
    fn clone(&self) -> Self {
        if let Some(stream) = &self.stream {
            if let Ok(stream) = stream.try_clone() {
                return Client {
                    stream: Some(stream),
                    sender: self.sender.clone(),
                };
            }
            return Client {
                stream: None,
                sender: None,
            };
        }
        return Client {
            stream: None,
            sender: None,
        };
    }
}

pub enum ClientAction {
    Connect,
    Publish,
    Subscribe,
}

pub type SenderClient = (
    ClientAction,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Sender<String>,
);

impl Client {
    pub fn init() -> Sender<SenderClient> {
        let (sender, receiver): (Sender<SenderClient>, Receiver<SenderClient>) = mpsc::channel();

        let mut client = Client {
            stream: None,
            sender: None,
        };

        thread::spawn(move || {
            for receive in receiver {
                match receive {
                    (
                        ClientAction::Connect,
                        Some(host),
                        Some(port),
                        Some(user),
                        Some(password),
                        Some(id_client),
                        sender_response,
                    ) => {
                        let response =
                            client.connect_to_server(host, port, user, password, id_client);
                        sender_response.send(response).unwrap();
                    }
                    (
                        ClientAction::Publish,
                        Some(message),
                        Some(topic),
                        None,
                        None,
                        None,
                        sender_response,
                    ) => {
                        client.publish_message(message, topic);
                    }
                    _ => panic!("Algo mal"),
                }
            }
        });

        sender
    }

    fn send_connect(&self, user: String, password: String, id_client: String) {
        let client_cloned = self.clone();
        match client_cloned.stream {
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
                println!("{:?}", bytes);
                stream.write_all(&bytes).unwrap();
            }
            None => panic!("No pude enviar"),
        }
    }

    pub fn publish_message(&self, message: String, topic: String) {
        let client_clone = self.clone();
        match client_clone.stream {
            Some(mut stream) => {
                let mut bytes = vec![
                    0x31, // Paquete publish QoS 0
                         //0x09,  Remaining Length
                ];
                self.add_topic_bytes(topic, &mut bytes);
                self.add_message_bytes(message, &mut bytes);
                let length = bytes.len();
                bytes.insert(1, (length - 1) as u8);
                stream.write_all(&bytes).unwrap();
            }
            None => panic!("No pude enviar"),
        }
    }

    fn check_connack_code(&self, code: u8) -> String {
        match code {
            0x00 => "Conexion realizada con exito".to_string(),
            0x01 => "Error: la version del protocolo no es compatible".to_string(),
            0x04 => "Error: los datos enviados no son correctos".to_string(),
            0x05 => "Error: no esta autorizado".to_string(),
            _ => "Error desconocido".to_string(),
        }
    }

    fn add_topic_bytes(&self, topic: String, bytes: &mut Vec<u8>) {
        if !topic.is_empty() {
            let topic_length = topic.len();
            let mut topic_in_bytes = topic.as_bytes().to_vec();
            bytes.push(0x00);
            bytes.push(topic_length as u8);
            bytes.append(&mut topic_in_bytes);
        }
    }

    fn add_message_bytes(&self, message: String, bytes: &mut Vec<u8>) {
        if !message.is_empty() {
            let message_length = message.len();
            let mut message_in_bytes = message.as_bytes().to_vec();
            bytes.push(0x00);
            bytes.push(message_length as u8);
            bytes.append(&mut message_in_bytes);
        }
    }

    pub fn connect_to_server(
        &mut self,
        host: String,
        port: String,
        user: String,
        password: String,
        id_client: String,
    ) -> String {
        let address = format!("{}:{}", host, port);
        match TcpStream::connect(address) {
            Ok(stream) => {
                let (tx, rx) = mpsc::channel::<u8>();
                let stream_copy = stream.try_clone().expect("Could not clone");
                self.stream = Some(stream);
                self.send_connect(user, password, id_client);
                thread::spawn(move || {
                    receive_responses_from_broker(stream_copy, tx);
                });
                let connack_code_received = rx.recv().unwrap();
                self.check_connack_code(connack_code_received)
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
                "La conexion no se ha podido establecer".to_string()
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
