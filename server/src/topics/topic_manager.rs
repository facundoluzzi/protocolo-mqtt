use crate::topics::topic::Topic;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub struct TopicManager {
    publish_sender: Sender<String>,
    subscribe_sender: Sender<String>,
    topics: Vec<Topic>
}

impl Clone for TopicManager {
    fn clone(&self) -> Self {
        let publish_sender = self.publish_sender.clone();
        let subscribe_sender = self.subscribe_sender.clone();
        let topics = &self.topics;
        Self { publish_sender, subscribe_sender, topics: topics.to_vec() }
    }
}

impl TopicManager {
    pub fn new() -> TopicManager {
        let (publish_tx, publish_rx): (Sender<String>, Receiver<String>) = mpsc::channel();
        let (subscribe_tx, subscribe_rx): (Sender<String>, Receiver<String>) = mpsc::channel();
        let topics: Vec<Topic> = Vec::new();

        let topic_manager = TopicManager { publish_sender: publish_tx, subscribe_sender: subscribe_tx, topics };

        thread::spawn(move || {
            for receive in publish_rx {
                // for topic in &topic_manager.topics {
                    
                // }
            }
        });

        thread::spawn(move || {
            for receive in subscribe_rx {}
        }); 

        topic_manager
    }

    pub fn get_publish_sender(&self) -> &Sender<String> {
        &self.publish_sender
    }
}
