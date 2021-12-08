use crate::enums::topic::publish_message::PublishMessage;
use crate::enums::topic::topic_actions::TopicAction;
use crate::enums::topic::topic_actions::TopicAction::{Add, Publish, Remove};
use crate::enums::user_manager::publish_message_user_manager::PublishMessageUserManager;
use crate::enums::user_manager::user_manager_action::UserManagerAction;
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

pub struct Topic {
    name: String,
    subscribers: HashMap<String, (Sender<UserManagerAction>, u8)>,
    retained_message: Option<PublishMessage>,
}

impl Topic {
    pub fn init(name: String) -> Sender<TopicAction> {
        let (topic_sender, topic_receiver): (Sender<TopicAction>, Receiver<TopicAction>) =
            mpsc::channel();
        let mut topic = Topic {
            name,
            subscribers: HashMap::new(),
            retained_message: None,
        };

        thread::spawn(move || {
            for message in topic_receiver {
                match message {
                    Add(action) => {
                        topic.add(
                            action.get_client_id(),
                            action.get_sender(),
                            action.get_qos(),
                        );
                    }
                    Remove(action) => {
                        topic.remove(action.get_client_id());
                    }
                    Publish(action) => {
                        let info = action.get_message();
                        let qos = action.get_qos();
                        let retained_message = action.get_retained_message();
                        if info.is_empty() && retained_message {
                            topic.retained_message = None;
                        } else if retained_message && qos == 0 {
                            let publish = PublishMessage::init(info.clone(), qos, true);
                            topic.retained_message = Some(publish);
                        }

                        topic.publish_msg(info, qos);
                    }
                }
            }
        });
        topic_sender
    }

    fn add(&mut self, client_id: String, sender: Sender<UserManagerAction>, qos_subscribe: u8) {
        self.publish_retained_message(client_id.to_owned(), sender.clone(), qos_subscribe);
        self.subscribers.insert(client_id, (sender, qos_subscribe));
    }

    fn remove(&mut self, subscriber: String) {
        self.subscribers.remove(&subscriber);
    }

    fn publish_retained_message(
        &self,
        client_id: String,
        sender: Sender<UserManagerAction>,
        qos_subscribe: u8,
    ) {
        if let Some(message) = &self.retained_message {
            let mut new_packet = message.get_message();
            let qos_publish = message.get_qos();
            if qos_subscribe + qos_publish < 2 {
                new_packet[0] &= 0b11111101;
            }
            let action = UserManagerAction::PublishMessageUserManager(
                PublishMessageUserManager::init(client_id, new_packet.clone()),
            );
            if let Err(msg) = sender.send(action) {
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
            let action = UserManagerAction::PublishMessageUserManager(
                PublishMessageUserManager::init(client_id.to_string(), new_packet.clone()),
            );
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
