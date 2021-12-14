use core::time;
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};

use crate::enums::publish_autosend::autosend_action::AutoSendAction;
use crate::enums::publish_autosend::publish_all_autosend::PublishAllAutoSend;
use crate::topic::publisher_writer::ChannelPublisherWriter;
use crate::topic::publisher_writer::PublisherSubscriberAction::PublishMessagePublisherSubscriber;

pub struct PublishAutoSend {
    publish_packets: HashMap<Vec<u8>, Vec<u8>>,
}

impl PublishAutoSend {
    pub fn init(sender_publisher_writer: Sender<ChannelPublisherWriter>) -> Sender<AutoSendAction> {
        let (sender, receiver): (Sender<AutoSendAction>, Receiver<AutoSendAction>) =
            mpsc::channel();
        let sender_to_return = sender.clone();
        let mut publish_autosend = PublishAutoSend {
            publish_packets: HashMap::new(),
        };
        std::thread::spawn(move || {
            for type_action in receiver {
                match type_action {
                    AutoSendAction::Add(operation) => {
                        publish_autosend
                            .add(operation.get_packet_identifier(), operation.get_publish());
                    }
                    AutoSendAction::Remove(operation) => {
                        publish_autosend.remove(operation.get_packet_identifier());
                    }
                    AutoSendAction::PublishAll(_) => {
                        for publish in publish_autosend.publish_packets.clone() {
                            if let Err(msg) = sender_publisher_writer.send((
                                PublishMessagePublisherSubscriber,
                                Some(publish.1),
                                None,
                            )) {
                                println!("Unexpected error: {}", msg);
                            };
                        }
                    }
                }
            }
        });
        std::thread::spawn(move || {
            loop{
                std::thread::sleep(time::Duration::from_secs(5));
                if let Err(error) = sender.send(AutoSendAction::PublishAll(PublishAllAutoSend::init()))
                {
                    println!("{:?}", error);
                };
            }
                
        });
        sender_to_return
    }

    pub fn add(&mut self, packet_identifier: Vec<u8>, receive: Vec<u8>) {
        let first_byte = receive[0] | 0b00001000;
        let mut packet = vec![first_byte];
        packet.append(&mut receive[1..receive.len()].to_vec());
        self.publish_packets.insert(packet_identifier, packet);
    }

    pub fn remove(&mut self, packet_identifier: Vec<u8>) {
        self.publish_packets.remove(&packet_identifier);
    }
}
