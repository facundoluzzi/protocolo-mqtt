use crate::usermanager::user_manager_action::UserManagerAction::PublishMessageUserManager;
use crate::usermanager::user_manager_types::ChannelUserManager;
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use crate::topics::topic_actions::TopicAction::{AddTopic, PublishMessage, RemoveTopic};
use crate::topics::topic_types::SenderTopicType;

pub struct Topic {
    name: String,
    subscribers: HashMap<String, (Sender<ChannelUserManager>, u8)>,
    retained_message: Option<SenderTopicType>,
}

impl Topic {
    pub fn init(name: String) -> Sender<SenderTopicType> {
        let (topic_sender, topic_receiver): (Sender<SenderTopicType>, Receiver<SenderTopicType>) =
            mpsc::channel();
        let mut topic = Topic {
            name,
            subscribers: HashMap::new(),
            retained_message: None,
        };

        thread::spawn(move || {
            for message in topic_receiver {
                let action_type = message.0;
                match action_type {
                    AddTopic => {
                        let info = message.1;
                        let sender_received = message.3;
                        let qos = message.4;

                        let sender = if let Some(sender) = sender_received {
                            sender
                        } else {
                            panic!("unexpected error");
                        };

                        let topic_received = if let Some(topic_received) = info {
                            topic_received
                        } else {
                            panic!("unexpected error");
                        };

                        topic.add(topic_received, sender, qos);
                    }
                    RemoveTopic => {
                        let info = message.1;
                        let topic_received = if let Some(topic_received) = info {
                            topic_received
                        } else {
                            panic!("unexpected error");
                        };

                        topic.remove(topic_received);
                    }
                    PublishMessage => {
                        let info = message.2.expect("Publish with None message");
                        let qos = message.4;

                        if let Some(retained_message) = message.5 {
                            if info.is_empty() && retained_message {
                                topic.retained_message = None;
                            } else if retained_message && qos == 0 {
                                topic.retained_message = Some((
                                    PublishMessage,
                                    None,
                                    Some(info.clone()),
                                    None,
                                    qos,
                                    Some(true),
                                ));
                            }
                        }

                        topic.publish_msg(info, qos);
                    }
                }
            }
        });
        topic_sender
    }

    fn add(&mut self, client_id: String, sender: Sender<ChannelUserManager>, qos_subscribe: u8) {
        self.publish_retained_message(client_id.to_owned(), sender.clone(), qos_subscribe);
        self.subscribers.insert(client_id, (sender, qos_subscribe));
    }

    fn remove(&mut self, subscriber: String) {
        self.subscribers.remove(&subscriber);
    }

    fn publish_retained_message(
        &self,
        client_id: String,
        sender: Sender<ChannelUserManager>,
        qos_subscribe: u8,
    ) {
        if let Some(message) = &self.retained_message {
            let mut new_packet = message.2.clone().expect("Publish with None message");
            let qos_publish = message.4;
            if qos_subscribe + qos_publish < 2 {
                new_packet[0] &= 0b11111101;
            }
            let tuple_for_publish = (
                PublishMessageUserManager,
                client_id,
                None,
                None,
                Some(new_packet.clone()),
            );
            if let Err(msg) = sender.send(tuple_for_publish) {
                println!("Unexpected error: {}", msg);
            };
        }
    }

    fn publish_msg(&self, packet: Vec<u8>, qos: u8) {
        for (client_id, (subscriber, qos_subscribe)) in &self.subscribers {
            let mut new_packet = packet.clone();
            if qos_subscribe + qos < 2 {
                new_packet[0] &= 0b11111101;
            }
            let tuple_for_publish = (
                PublishMessageUserManager,
                client_id.to_string(),
                None,
                None,
                Some(new_packet.clone()),
            );
            if let Err(msg) = subscriber.send(tuple_for_publish) {
                println!("Unexpected error: {}", msg);
            };
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn equals(&self, other_topic: String) -> bool {
        self.name == other_topic
    }
}
