use crate::enums::topic::add_topic::AddTopic;
use crate::enums::topic::publish_message::PublishMessage;
use crate::enums::topic::remove_topic::RemoveTopic;
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
    /// Constructor del struct. Lanza un thread escuchando por eventos.
    /// Los eventos pueden ser Add topic, remove topic o publish message.
    pub fn init(name: String) -> Sender<TopicAction> {
        let (topic_sender, topic_receiver): (Sender<TopicAction>, Receiver<TopicAction>) =
            mpsc::channel();
        let topic = Topic {
            name,
            subscribers: HashMap::new(),
            retained_message: None,
        };
        topic.throw_thread_to_listen_events(topic_receiver);
        topic_sender
    }

    fn throw_thread_to_listen_events(mut self, receiver: Receiver<TopicAction>) {
        thread::spawn(move || {
            for message in receiver {
                match message {
                    Add(action) => self.add(action),
                    Remove(action) => self.remove(action),
                    Publish(action) => self.publish(action),
                }
            }
        });
    }

    fn add(&mut self, add_topic: AddTopic) {
        let client_id = add_topic.get_client_id();
        let sender = add_topic.get_sender();
        let qos_subscribe = add_topic.get_qos();
        self.publish_retained_message(client_id.to_owned(), sender.clone(), qos_subscribe);
        self.subscribers.insert(client_id, (sender, qos_subscribe));
    }

    fn remove(&mut self, remove_topic: RemoveTopic) {
        self.subscribers.remove(&remove_topic.get_client_id());
    }

    fn publish(&mut self, publish_message: PublishMessage) {
        let packet = publish_message.get_packet();
        let qos = publish_message.get_qos();
        let message = publish_message.get_message();
        let retained_message = publish_message.get_retained_message();
        if retained_message {
            if packet.is_empty() {
                self.retained_message = None;
            } else {
                let publish = PublishMessage::init(packet.clone(), qos, true, message.to_string());
                self.retained_message = Some(publish);
            }
        }

        self.publish_msg(packet, qos, message);
    }

    fn publish_retained_message(
        &self,
        client_id: String,
        sender: Sender<UserManagerAction>,
        qos_subscribe: u8,
    ) {
        if let Some(message) = &self.retained_message {
            let mut new_packet = message.get_packet();
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

    fn publish_msg(&self, packet: Vec<u8>, qos: u8, message: String) {
        for (client_id, (subscriber, qos_subscribe)) in &self.subscribers {
            let mut new_packet = packet.clone();
            if qos_subscribe + qos < 2 {
                new_packet[0] &= 0b11111101;
            }

            if qos == 1 && *qos_subscribe == 0 {
                let message_length = message.len();
                let index_to_delete = packet.len() - message_length - 2;
                new_packet.remove(index_to_delete);
                new_packet.remove(index_to_delete);
                new_packet[1] -= 2;
            }
            let action = UserManagerAction::PublishMessageUserManager(
                PublishMessageUserManager::init(client_id.to_string(), new_packet.clone()),
            );

            if let Err(msg) = subscriber.send(action) {
                println!("Unexpected error: {}", msg);
            };
        }
    }

    /// obtiene el nombre del topic
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    /// determina si dos topicos son iguales
    pub fn equals(&self, other_topic: String) -> bool {
        self.name == other_topic
    }
}
