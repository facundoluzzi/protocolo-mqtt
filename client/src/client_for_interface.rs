use crate::helper::stream::stream_handler::StreamAction::ReadStream;
use crate::helper::stream::stream_handler::StreamType;
use crate::packet::packet_manager::PacketManager;
use crate::packet::sender_type::ClientSender;
use crate::packet::sender_type::InterfaceSender;

use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

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
        Client {
            stream: None,
            sender: None,
            sender_stream: None,
        }
    }
}

pub enum ClientAction {
    Connect,
    Publish,
    Subscribe,
}

type SenderForReading = Sender<(Sender<StreamType>, gtk::glib::Sender<ClientSender>)>;
type ReceiverForReading = Receiver<(Sender<StreamType>, gtk::glib::Sender<ClientSender>)>;

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
            SenderForReading,
            ReceiverForReading,
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
                        Some(sender_stream) => match publish.send_publish(sender_stream.clone()) {
                            Ok(_result_ok) => {
                                println!("Ok");
                            }
                            Err(err) => {
                                println!("err: {}", err);
                            }
                        },
                        None => {
                            println!("Unexpected error");
                        }
                    },
                    InterfaceSender::Subscribe(subscribe) => match client.sender_stream.clone() {
                        Some(sender_stream) => {
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
                    InterfaceSender::Unsubscribe(unsubscribe) => match client.sender_stream.clone()
                    {
                        Some(sender_stream) => {
                            match unsubscribe.send_unsubscribe(sender_stream.clone()) {
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
        let response = packet_manager.process_message(bytes);

        if let Some(client_sender) = response {
            sender.send(client_sender).unwrap();
        };

        Ok(())
    }
}
