use crate::enums::topic::topic_actions::TopicAction;
use crate::enums::topic_manager::topic_message::TypeMessage;

use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub struct TopicManager {
    publisher_subscriber_sender: Sender<TypeMessage>,
    topics: HashMap<String, Sender<TopicAction>>,
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
    pub fn init() -> Sender<TypeMessage> {
        let (publisher_subscriber_sender, publisher_subscriber_receiver): (
            Sender<TypeMessage>,
            Receiver<TypeMessage>,
        ) = mpsc::channel();
        let sender_to_return = publisher_subscriber_sender.clone();

        let topics: HashMap<String, Sender<TopicAction>> = HashMap::new();
        let mut topic_manager = TopicManager {
            publisher_subscriber_sender,
            topics,
        };

        thread::spawn(move || {
            for publish_subscriber in publisher_subscriber_receiver {
                match publish_subscriber {
                    TypeMessage::Publisher(publisher) => {
                        topic_manager.topics = publisher.publish(topic_manager.topics.clone());
                    }
                    TypeMessage::Subscriber(mut subscriber) => {
                        topic_manager.topics = subscriber.subscribe(topic_manager.topics.clone());
                    }
                    TypeMessage::Unsubscriber(mut unsubscriber) => {
                        topic_manager.topics =
                            unsubscriber.unsubscribe(topic_manager.topics.clone());
                    }
                    TypeMessage::UnsubscriberAll(mut unsubscriber_all) => {
                        unsubscriber_all.unsubscribe_all(topic_manager.topics.clone())
                    }
                }
            }
        });
        sender_to_return
    }
}
