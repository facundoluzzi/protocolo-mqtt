use crate::sender_types::sender_type::ClientSender;
use crate::sender_types::sender_type::InterfaceSender;
use crate::stream::stream_handler::StreamAction::ReadStream;
use crate::{packet_manager::PacketManager, stream::stream_handler::StreamType};
use std::{net::TcpStream, thread};

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

pub struct Client {
    stream: Option<TcpStream>,
    sender: Option<Sender<(usize, Vec<String>)>>,
    sender_stream: Option<Sender<StreamType>>,
}

impl Clone for Client {
    fn clone(&self) -> Self {
        if let Some(stream) = &self.stream {
            if let Ok(stream) = stream.try_clone() {
                return Client {
                    stream: Some(stream),
                    sender: self.sender.clone(),
                    sender_stream: None,
                };
            }
            return Client {
                stream: None,
                sender: None,
                sender_stream: None,
            };
        }
        return Client {
            stream: None,
            sender: None,
            sender_stream: None,
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
    Option<Sender<String>>,
);

impl Client {
    pub fn init() -> Sender<InterfaceSender> {
        let (event_sender, event_receiver): (Sender<InterfaceSender>, Receiver<InterfaceSender>) =
            mpsc::channel();

        let mut client = Client {
            stream: None,
            sender: None,
            sender_stream: None,
        };

        let (sender_to_start_reading, receiver_to_start_reading): (
            Sender<(Sender<StreamType>, gtk::glib::Sender<ClientSender>)>,
            Receiver<(Sender<StreamType>, gtk::glib::Sender<ClientSender>)>,
        ) = mpsc::channel();

        thread::spawn(move || {
            for event in event_receiver {
                match event {
                    InterfaceSender::Connect(connect) => {
                        let sender_stream = connect.connect_to_server();
                        if let Ok(sender) = sender_stream {
                            client.sender_stream = Some(sender.clone());
                            sender_to_start_reading
                                .send((sender.clone(), connect.get_gtk_sender()))
                                .unwrap();
                        }
                    }
                    InterfaceSender::Publish(publish) => match client.sender_stream.clone() {
                        Some(sender_stream) => {
                            println!("\n\n Publish \n\n");
                            match publish.send_publish(sender_stream.clone()) {
                                Ok(_result_ok) => {
                                    println!("Ok");
                                }
                                Err(err) => {
                                    println!("err: {}", err);
                                }
                            }
                        }
                        None => {
                            println!("Unexpected error");
                        }
                    },
                    InterfaceSender::Subscribe(subscribe) => match client.sender_stream.clone() {
                        Some(sender_stream) => {
                            println!("\n\n Subscribe \n\n");
                            match subscribe.send_suscribe(sender_stream.clone()) {
                                Ok(_result_ok) => {
                                    println!("Ok");
                                }
                                Err(err) => {
                                    println!("err: {}", err);
                                }
                            }
                        }
                        None => {
                            println!("Unexpected error")
                        }
                    },
                };
            }
        });

        thread::spawn(move || {
            let (sender_stream, sender_gtk) = receiver_to_start_reading.recv().unwrap();

            let (packet_sender, packet_receiver) = mpsc::channel::<Vec<u8>>();

            loop {
                println!("\n\n\n Leyendo \n\n\n");
                let message_sent =
                    sender_stream
                        .clone()
                        .send((ReadStream, None, Some(packet_sender.clone())));

                if let Err(_msg) = message_sent {
                } else if let Ok(packet) = packet_receiver.recv() {
                    let packet_u8: &[u8] = &packet;
                    if let Err(err) = Client::process_packet(packet_u8, sender_gtk.clone()) {
                        println!("err: {}", err);
                        break;
                    }
                }
            }
        });

        event_sender
    }

    fn process_packet(bytes: &[u8], sender: gtk::glib::Sender<ClientSender>) -> Result<(), String> {
        let packet_manager = PacketManager::new();
        let response = packet_manager.process_message(&bytes);

        match response {
            Some(clientSender) => sender.send(clientSender).unwrap(),
            None => {}
        };

        Ok(())
    }
}
