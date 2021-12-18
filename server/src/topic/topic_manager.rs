use crate::enums::topic::topic_actions::TopicAction;
use crate::enums::topic_manager::topic_message::TypeMessage;

use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

/// Contiene un hash map de topicos
pub struct TopicManager {
    topics: HashMap<String, Sender<TopicAction>>,
}

impl Clone for TopicManager {
    fn clone(&self) -> Self {
        Self {
            topics: self.topics.clone(),
        }
    }
}

impl TopicManager {
    /// Lanza un thread para quedarse escuchando por eventos.
    /// Los eventos pueden ser Publisher, Subscriber, Unsubscriber, UnsubscriberAll
    pub fn init() -> Sender<TypeMessage> {
        let (publisher_subscriber_sender, event_receiver): (
            Sender<TypeMessage>,
            Receiver<TypeMessage>,
        ) = mpsc::channel();
        let topics: HashMap<String, Sender<TopicAction>> = HashMap::new();
        let topic_manager = TopicManager { topics };
        topic_manager.throw_thread_to_listen_events(event_receiver);
        publisher_subscriber_sender
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
