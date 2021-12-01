
use crate::helper::publisher_subscriber_code::PublisherSubscriberCode;
use crate::paquetes::publisher_suscriber::PublisherSuscriber;
use crate::topics::topic::Topic;
use crate::wildcard::verify_wildcard;
use crate::wildcard::wildcard::Wildcard;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::topics::topic_types::SenderTopicType;
use crate::topics::topic_actions::TopicAction::{ AddTopic, RemoveTopic, PublishMessage };

use super::topic_actions::TopicAction;

pub struct TopicManager {
    publisher_subscriber_sender: Sender<PublisherSuscriber>,
    topics: HashMap<String, Sender<SenderTopicType>>
}

impl Clone for TopicManager {
    fn clone(&self) -> Self {
        let publisher_subscriber_sender = self.publisher_subscriber_sender.clone();
        Self {
            publisher_subscriber_sender,
            topics: self.topics.clone(),
        }
    }
}

impl TopicManager {
    pub fn init() -> Sender<PublisherSuscriber> {
        let (publisher_subscriber_sender, publisher_subscriber_receiver): (
            Sender<PublisherSuscriber>,
            Receiver<PublisherSuscriber>,
        ) = mpsc::channel();
        let sender_to_return = publisher_subscriber_sender.clone();
        
        let topics: HashMap<String, Sender<SenderTopicType>> = HashMap::new();
        let mut topic_manager = TopicManager {
            publisher_subscriber_sender,
            topics,
        };
        thread::spawn(move || {
            for publish_suscriber in publisher_subscriber_receiver {
                match publish_suscriber.get_packet_type() {
                    PublisherSubscriberCode::Publisher => {topic_manager.publish_msg(publish_suscriber.get_topic(), publish_suscriber.get_message());
        },
                    PublisherSubscriberCode::Subscriber => {
                        let subscriber = publish_suscriber.get_sender().unwrap();
                        let topic_name = publish_suscriber.get_topic();
                        let client_id = publish_suscriber.get_client_id();
                        if let Some(wilcard) = verify_wildcard::get_wilcard(topic_name.to_owned()){
                            topic_manager.subscribe_with_wilcard(wilcard, subscriber.clone(), publish_suscriber.get_client_id());
                        } else {
                            topic_manager.subscribe(topic_name.to_owned(), client_id.to_owned(), subscriber);
                        }
                    }
                };
            }
        });
        sender_to_return
    }

    fn publish_msg(&self, topic_name: String, message: String) {
        if let Some(topic_sender) = &self.topics.get(&topic_name) {
            topic_sender.send((PublishMessage, message, None)).unwrap();
        }
    }

    fn subscribe(&mut self, topic_name: String, client_id: String, sender_subscriber: Sender<String>) {
        if let Some(topic_sender) = self.topics.get(&topic_name.to_owned()) {
            topic_sender.send((AddTopic, client_id.to_owned(), Some(sender_subscriber))).unwrap();
        } else {
            let sender_topic = Topic::new(topic_name.to_owned());
            self.topics.insert(topic_name.to_owned(), sender_topic.clone());
            sender_topic.send((AddTopic, client_id.to_owned(), Some(sender_subscriber))).unwrap();
        }
    }

    pub fn subscribe_with_wilcard(&self, wilcard: Wildcard, sender_subscribe: Sender<String>, client_id: String) {
        for (topic_name, topic_sender) in &self.topics {
            if wilcard.verify_topic(topic_name.to_owned()) {
                topic_sender.send((AddTopic, client_id.to_owned(), Some(sender_subscribe.clone()))).unwrap();
            }
        }   
    }
}