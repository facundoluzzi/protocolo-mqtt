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
        let (publisher_subscriber_sender, event_receiver): (
            Sender<TypeMessage>,
            Receiver<TypeMessage>,
        ) = mpsc::channel();
        let sender_to_return = publisher_subscriber_sender.clone();

        let topics: HashMap<String, Sender<TopicAction>> = HashMap::new();
        let topic_manager = TopicManager {
            publisher_subscriber_sender,
            topics,
        };
        topic_manager.throw_thread_to_listen_events(event_receiver);
        sender_to_return
    }

    fn throw_thread_to_listen_events(mut self, event_receiver: Receiver<TypeMessage>) {
        thread::spawn(move || {
            for event in event_receiver {
                let topics = self.topics.clone();
                match event {
                    TypeMessage::Publisher(publisher) => {
                        self.topics = publisher.publish(topics);
                    }
                    TypeMessage::Subscriber(mut subscriber) => {
                        self.topics = subscriber.subscribe(topics);
                    }
                    TypeMessage::Unsubscriber(mut unsubscriber) => {
                        self.topics = unsubscriber.unsubscribe(topics);
                    }
                    TypeMessage::UnsubscriberAll(mut unsubscriber_all) => {
                        self.topics = unsubscriber_all.unsubscribe_all(topics);
                    }
                }
            }
        });
    }
}
