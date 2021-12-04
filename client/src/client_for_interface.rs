use std::{
    io::{Read, Write},
    net::TcpStream,
    thread,
};

use crate::{
    packet_builder::{build_bytes_for_connect, build_bytes_for_publish, build_bytes_for_suscribe},
    packet_manager::{PacketManager, ResponsePacket},
    trait_paquetes::Paquetes,
};
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
    Option<bool>,
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
                        None,
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
                        Some(is_qos_0),
                        sender_response,
                    ) => {
                        client.publish_into_topic(message, topic, is_qos_0);
                    }
                    (
                        ClientAction::Subscribe,
                        Some(topic),
                        None,
                        None,
                        None,
                        None,
                        Some(is_qos_0),
                        sender_response,
                    ) => {
                        let response = client.subscribe_to_topic(topic, is_qos_0);
                        sender_response.send(response).unwrap();
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
                let connect_bytes = build_bytes_for_connect(user, password, id_client);
                stream.write_all(&connect_bytes).unwrap();
            }
            None => panic!("No pude enviar"),
        }
    }

    pub fn send_suscribe(&self, topic: &String, is_qos_0: bool, stream: Option<TcpStream>) {
        match stream {
            Some(mut stream) => {
                let suscribe_bytes = build_bytes_for_suscribe(topic, is_qos_0);
                stream.write_all(&suscribe_bytes).unwrap();
            }
            None => panic!("No pude enviar"),
        }
    }

    pub fn subscribe_to_topic(&self, topic: String, is_qos_0: bool) -> String {
        let client_clone = self.clone();
        let client_clone_2 = self.clone();
        let (tx, rx) = mpsc::channel::<u8>();
        self.send_suscribe(&topic, is_qos_0, client_clone.stream);
        thread::spawn(move || {
            receive_responses_from_broker(client_clone_2.stream.unwrap(), tx);
        });
        let suscribe_code_received = rx.recv().unwrap();
        self.check_suback_code(suscribe_code_received, topic)
    }

    fn check_suback_code(&self, code: u8, topic: String) -> String {
        match code {
            0x00 => format!("Suscripcion realizada a {}", topic),
            0x01 => format!("Suscripcion realizada a {}", topic),
            _ => "Error en suscripcion".to_string(),
        }
    }

    pub fn send_publish(
        &self,
        message: String,
        topic: String,
        is_qos_0: bool,
        stream: Option<TcpStream>,
    ) {
        match stream {
            Some(mut stream) => {
                let publish_bytes = build_bytes_for_publish(topic, message, is_qos_0);
                stream.write_all(&publish_bytes).unwrap();
            }
            None => panic!("No pude enviar"),
        }
    }

    pub fn publish_into_topic(&self, message: String, topic: String, is_qos_0: bool) {
        let client_clone = self.clone();
        let client_clone_2 = self.clone();
        let (tx, rx) = mpsc::channel::<u8>();
        self.send_publish(message, topic, is_qos_0, client_clone.stream);
        if !is_qos_0 {
            thread::spawn(move || {
                receive_responses_from_broker(client_clone_2.stream.unwrap(), tx);
            });
            let suscribe_code_received = rx.recv().unwrap();
            println!("{:?}", suscribe_code_received);
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
    let mut data = vec![0_u8; 100];
    match stream.read(&mut data) {
        Ok(_) => {
            let packet_manager = PacketManager::new();
            let packet_received = packet_manager.process_message(&data);
            match packet_received.get_type() {
                ResponsePacket::Connack => {
                    let code = packet_received.get_status_code();
                    channel_producer.send(code).unwrap();
                }
                ResponsePacket::Suback => {
                    let code = packet_received.get_status_code();
                    channel_producer.send(code).unwrap();
                }
                ResponsePacket::Puback => {
                    let code = packet_received.get_status_code();
                    channel_producer.send(code).unwrap();
                }
                _ => println!("Received Default"),
            }
        }
        Err(e) => {
            println!("Failed to receive data: {}", e);
        }
    }
}
