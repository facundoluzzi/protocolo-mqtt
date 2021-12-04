use crate::usermanager::user_manager_action::UserManagerAction::PublishMessageUserManager;
use crate::usermanager::user_manager_types::ChannelUserManager;
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use crate::topics::topic_actions::TopicAction::{AddTopic, PublishMessage, RemoveTopic};
use crate::topics::topic_types::SenderTopicType;

pub struct Topic {
    name: String,
    subscribers: HashMap<String, Sender<ChannelUserManager>>,
}

impl Topic {
    pub fn new(name: String) -> Sender<SenderTopicType> {
        let (topic_sender, topic_receiver): (Sender<SenderTopicType>, Receiver<SenderTopicType>) =
            mpsc::channel();
        let mut topic = Topic {
            name,
            subscribers: HashMap::new(),
        };

        thread::spawn(move || {
            for message in topic_receiver {
                let action_type = message.0;
                let info = message.1;
                match action_type {
                    AddTopic => {
                        let sender = if let Some(sender) = message.2 {
                            sender
                        } else {
                            panic!("unexpected error");
                        };
                        topic.add(info, sender);
                    }
                    RemoveTopic => {
                        topic.remove(info);
                    }
                    PublishMessage => {
                        topic.publish_msg(info);
                    }
                }
            }
        });
        topic_sender
    }

    fn add(&mut self, client_id: String, sender: Sender<ChannelUserManager>) {
        self.subscribers.insert(client_id, sender);
    }

    fn remove(&mut self, subscriber: String) {
        self.subscribers.remove(&subscriber);
    }

    fn publish_msg(&self, message: String) {
        for (client_id, subscriber) in &self.subscribers {
            if let Err(_msg) = subscriber.send((
                PublishMessageUserManager,
                client_id.to_string(),
                None,
                None,
                Some(message.to_string()),
            )) {
                println!("Error al publicar el mensaje")
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
