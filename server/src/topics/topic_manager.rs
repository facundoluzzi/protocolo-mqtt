use crate::topics::topic_types::SenderTopicType;
use crate::topics::topic_types::TypeTopicManager;

use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub struct TopicManager {
    publisher_subscriber_sender: Sender<TypeTopicManager>,
    topics: HashMap<String, Sender<SenderTopicType>>,
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
    pub fn init() -> Sender<TypeTopicManager> {
        let (publisher_subscriber_sender, publisher_subscriber_receiver): (
            Sender<TypeTopicManager>,
            Receiver<TypeTopicManager>,
        ) = mpsc::channel();
        let sender_to_return = publisher_subscriber_sender.clone();

        let topics: HashMap<String, Sender<SenderTopicType>> = HashMap::new();
        let mut topic_manager = TopicManager {
            publisher_subscriber_sender,
            topics,
        };

        thread::spawn(move || {
            for publish_subscriber in publisher_subscriber_receiver {
                match publish_subscriber {
                    TypeTopicManager::Publisher(publisher) => {
                        publisher.publish(topic_manager.topics.clone());
                    }
                    TypeTopicManager::Subscriber(mut subscriber) => {
                        topic_manager.topics = subscriber.subscribe(topic_manager.topics.clone());
                    }
                    TypeTopicManager::Unsubscriber(mut unsubscriber) => {
                        unsubscriber.unsubscribe(topic_manager.topics.clone())
                    }
                    TypeTopicManager::UnsubscriberAll(mut unsubscriber_all) => {
                        unsubscriber_all.unsubscribe_all(topic_manager.topics.clone())
                    }
                }
            }
        });
        sender_to_return
    }
}
