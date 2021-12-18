use crate::helper::stream::stream_handler::StreamAction::ReadStream;
use crate::packet::input::connect::Connect;
use crate::packet::input::pingreq::Pingreq;
use crate::packet::packet_manager::PacketManager;
use crate::packet::sender_type::ClientSender;
use crate::packet::sender_type::InterfaceSender;
use crate::types::SenderForServer;
use crate::types::StreamType;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SendError;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

pub struct Client {
    sender_stream: Option<Sender<StreamType>>,
    signal_sender: Option<Sender<bool>>,
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
            sender_stream: None,
            signal_sender: None,
        };

        thread::spawn(move || {
            for event in event_receiver {
                match event {
                    InterfaceSender::Connect(connect) => {
                        let sender_stream = connect.connect_to_server();
                        if let Ok(sender) = sender_stream {
                            client.sender_stream = Some(sender.clone());

                            let (sender_for_ping, receiver_for_ping) = mpsc::channel::<bool>();

                            if !connect.keep_alive_is_empty() {
                                client.signal_sender = Some(sender_for_ping.clone());
                                Client::start_to_send_pingreq(
                                    &connect,
                                    sender.clone(),
                                    receiver_for_ping,
                                );
                            }
                            Client::start_to_read(sender.clone(), connect.get_gtk_sender());
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
                    InterfaceSender::Disconnect(disconnect) => match client.sender_stream.clone() {
                        Some(sender_stream) => {
                            match disconnect.send_disconnect(sender_stream.clone()) {
                                Ok(_result_ok) => {
                                    if let Some(signal_sender) = client.signal_sender.clone() {
                                        if let Err(err) = signal_sender.send(false) {
                                            println!("{}", err);
                                        } else {
                                            client.signal_sender = None;
                                        }
                                    }
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
                    InterfaceSender::PubackToSend(puback_to_send) => {
                        match client.sender_stream.clone() {
                            Some(sender_stream) => {
                                if puback_to_send.send_puback(sender_stream.clone()).is_err() {
                                    println!("Error sending PUBACK")
                                };
                            }
                            None => {
                                println!("Unexpected error");
                            }
                        }
                    }
                }
            }
        });

        event_sender
    }

    fn process_packet(
        bytes: &[u8],
        sender: gtk::glib::Sender<ClientSender>,
    ) -> Result<(), SendError<ClientSender>> {
        let packet_manager = PacketManager::new();
        let response = packet_manager.process_message(bytes);

        if let Some(client_sender) = response {
            sender.send(client_sender)?;
        };

        Ok(())
    }

    fn is_empty_packet(packet: Vec<u8>) -> bool {
        !packet.into_iter().any(|element| element != 0)
    }

    fn start_to_read(
        sender_stream: Sender<StreamType>,
        sender_gtk: gtk::glib::Sender<ClientSender>,
    ) {
        thread::spawn(move || {
            let (packet_sender, packet_receiver) = mpsc::channel::<Vec<u8>>();

            loop {
                let message_sent =
                    sender_stream
                        .clone()
                        .send((ReadStream, None, Some(packet_sender.clone())));

                if let Err(_msg) = message_sent {
                } else if let Ok(packet) = packet_receiver.recv() {
                    let empty_packet = Client::is_empty_packet(packet.clone());
                    if empty_packet {
                        break;
                    }
                    let packet_u8: &[u8] = &packet;
                    if let Err(err) = Client::process_packet(packet_u8, sender_gtk.clone()) {
                        println!("err: {}", err);
                        break;
                    }
                }
            }
        });
    }

    fn start_to_send_pingreq(
        connect: &Connect,
        sender: SenderForServer,
        receiver_for_ping: Receiver<bool>,
    ) {
        let pingreq = Pingreq::init(connect.get_keep_alive());
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(pingreq.get_interval() as u64));
            let keep_sending = receiver_for_ping.try_recv();
            if keep_sending.is_ok() {
                return;
            }
            match pingreq.send_pingreq(sender.clone()) {
                Ok(_result) => {
                    println!("Mando pingreq");
                }
                Err(err) => {
                    println!("err: {}", err);
                }
            }
        });
    }
}
