use crate::topics::topic::Topic;
use crate::topics::topic_actions::TopicAction::AddTopic;
use crate::topics::topic_types::SenderTopicType;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

use crate::usermanager::user_manager_types::ChannelUserManager;
use crate::wildcard::wildcard_handler::Wildcard;

pub struct Subscriber {
    client_id: String,
    topic: String,
    sender_user_manager: Sender<ChannelUserManager>,
    wildcard: Option<Wildcard>,
    qos: u8
}

impl Subscriber {
    pub fn init(
        client_id: String,
        topic: String,
        sender_user_manager: Sender<ChannelUserManager>,
        wildcard: Option<Wildcard>,
        qos: u8
    ) -> Subscriber {
        Subscriber {
            client_id,
            topic,
            sender_user_manager,
            wildcard,
            qos
        }
    }

    pub fn subscribe(&mut self, topics: HashMap<String, Sender<SenderTopicType>>) -> HashMap<String, Sender<SenderTopicType>> {
        let new_topic = match &self.wildcard {
            Some(wildcard) => {
                self.subscribe_with_wilcard(wildcard.clone(), topics)
            },
            None => {
                self.subscribe_without_wilcard(topics)
            }
        };
        
        new_topic
    }

    fn subscribe_without_wilcard(&mut self, mut topics: HashMap<String, Sender<SenderTopicType>>) -> HashMap<String, Sender<SenderTopicType>> {
        match topics.get(&self.topic) {
            Some(topic_sender) => {
                topic_sender
                .send((AddTopic, Some(self.client_id.to_string()), None, Some(self.sender_user_manager.clone())))
                .unwrap();
            },
            None => {
                let sender_topic = Topic::init(self.topic.to_owned());
                topics.insert(self.topic.to_owned(), sender_topic.clone());
                sender_topic
                    .send((AddTopic, Some(self.client_id.to_string()), None, Some(self.sender_user_manager.clone())))
                    .unwrap();
            }
        }

        topics
    }

    pub fn subscribe_with_wilcard(&self, wilcard: Wildcard, topics: HashMap<String, Sender<SenderTopicType>>) -> HashMap<String, Sender<SenderTopicType>> {
        for (topic_name, topic_sender) in &topics {
            if wilcard.verify_topic(topic_name.to_owned()) {
                topic_sender
                    .send((
                        AddTopic,
                        Some(self.client_id.to_owned()),
                        None,
                        Some(self.sender_user_manager.clone()),
                    ))
                    .unwrap();
            }
        }
        topics
    }

    pub fn get_client_id(&self) -> String {
        self.client_id.to_string()
    }

    pub fn get_topic(&self) -> String {
        self.topic.to_string()
    }

    pub fn get_sender_user_manager(&self) -> Sender<ChannelUserManager> {
        self.sender_user_manager.clone()
    }

    pub fn get_wildcard(&self) -> Option<Wildcard> {
        match &self.wildcard {
            Some(wildcard) => Some(wildcard.clone()),
            None => None,
        }
    }

    pub fn get_qos(&self) -> u8 {
        self.qos
    }
}
