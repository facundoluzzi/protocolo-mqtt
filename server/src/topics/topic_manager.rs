use crate::paquetes::publish::Publish;
use crate::paquetes::subscribe::Subscribe;
use crate::topics::topic::Topic;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub struct TopicManager {
    publisher_subscriber_sender: Sender<String>,
    topics: Vec<Topic>,
}

impl Clone for TopicManager {
    fn clone(&self) -> Self {
        // let topics = &self.topics;
        // Self { publisher_subscriber_sender, subscribe_sender, topics: topics.to_vec() }
        let publisher_subscriber_sender = self.publisher_subscriber_sender.clone();
        Self {
            publisher_subscriber_sender,
            topics: self.topics.clone(),
        }
    }
}

impl TopicManager {
    pub fn new() -> TopicManager {
        let (publisher_subscriber_sender, publisher_subscriber_receiver): (
            Sender<String>,
            Receiver<String>,
        ) = mpsc::channel();
        let topics: Vec<Topic> = Vec::new();

        let topic_manager = TopicManager {
            publisher_subscriber_sender,
            topics,
        };
        let mut topics_copy = topic_manager.topics.clone();

        thread::spawn(move || {
            for sub in publisher_subscriber_receiver {
                // hay que crear un struct PublisherSubscriber que tenga el tipo, recibimos un struct de ese tipo acá.
                // Dependiendo de que haga, lo podemos mandar a dos threads diferentes o no. Pero nos puede servir para bloquear
                // los publishers mientras hayan subscripciones en proceso o lo opuesto.
                topics_copy.push(Topic::new(sub));
            }
        });

        topic_manager
    }

    pub fn get_sender(&self) -> Sender<String> {
        self.publisher_subscriber_sender.clone()
    }
}
