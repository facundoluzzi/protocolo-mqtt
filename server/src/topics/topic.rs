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

                        topic.add(topic_received, sender);
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
                        let message = if let Some(message) = info {
                            message
                        } else {
                            panic!("unexpected error");
                        };

                        topic.publish_msg(message);
                    }
                }
            }
        });
        topic_sender
    }

    fn add(&mut self, client_id: String, sender: Sender<ChannelUserManager>) {
        self.subscribers.insert(client_id, (sender, 0));
    }

    fn remove(&mut self, subscriber: String) {
        self.subscribers.remove(&subscriber);
    }

    fn publish_msg(&self, packet: Vec<u8>) {
        for (client_id, (subscriber, _qos)) in &self.subscribers {
            let tuple_for_publish = (
                PublishMessageUserManager,
                client_id.to_string(),
                None,
                None,
                Some(packet.clone()),
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
