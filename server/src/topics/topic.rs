use crate::usermanager::publishmessageusermanager::PublishMessageUserManager;
use crate::usermanager::user_manager_action::UserManagerAction;
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use crate::topics::topic_actions::TopicAction::{AddTopic, PublishMessage, RemoveTopic};
use crate::topics::topic_types::SenderTopicType;

pub struct Topic {
    name: String,
    subscribers: HashMap<String, (Sender<UserManagerAction>, u8)>,
}

impl Topic {
    pub fn init(name: String) -> Sender<SenderTopicType> {
        let (topic_sender, topic_receiver): (Sender<SenderTopicType>, Receiver<SenderTopicType>) =
            mpsc::channel();
        let mut topic = Topic {
            name,
            subscribers: HashMap::new(),
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
                        let info = message.2;
                        let qos = message.4;
                        let message = if let Some(message) = info {
                            message
                        } else {
                            panic!("unexpected error");
                        };

                        topic.publish_msg(message, qos);
                    }
                }
            }
        });
        topic_sender
    }

    fn add(&mut self, client_id: String, sender: Sender<UserManagerAction>, qos: u8) {
        self.subscribers.insert(client_id, (sender, qos));
    }

    fn remove(&mut self, subscriber: String) {
        self.subscribers.remove(&subscriber);
    }

    fn publish_msg(&self, packet: Vec<u8>, qos: u8) {
        for (client_id, (subscriber, qos_subscribe)) in &self.subscribers {
            let mut new_packet = packet.clone();
            if qos_subscribe + qos < 2 {
                new_packet[0] = new_packet[0] & 0b11111101;
            }
            // let tuple_for_publish = (
            //     PublishMessageUserManager,
            //     client_id.to_string(),
            //     None,
            //     None,
            //     Some(new_packet.clone()),
            // );
            let action = UserManagerAction::PublishMessageUserManager::init(client_id.to_string(), new_packet.clone());
            if let Err(msg) = subscriber.send(action) {
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
