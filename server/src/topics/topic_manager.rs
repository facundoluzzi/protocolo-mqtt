use crate::paquetes::publish::Publish;
use crate::paquetes::subscribe::Subscribe;
use crate::topics::topic::Topic;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub struct TopicManager {
    publish_sender: Sender<Publish>,
    subscribe_sender: Sender<Subscribe>,
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
        let (publish_tx, publish_rx): (Sender<Publish>, Receiver<Publish>) = mpsc::channel();
        let (subscribe_tx, subscribe_rx): (Sender<Subscribe>, Receiver<Subscribe>) = mpsc::channel();
        let topics: Vec<Topic> = Vec::new();

        let topic_manager = TopicManager { publish_sender: publish_tx, subscribe_sender: subscribe_tx, topics };
        let topics_copy = topic_manager.topics.clone();

        thread::spawn(move || {
            for receive in publish_rx {
                for topic in &topics_copy {
                    if topic.clone().equals(receive.get_name()){
                        topic.publish_msg(receive.get_publish_message());
                    }
                }
            }
        });

        thread::spawn(move || {
            for receive in subscribe_rx {
                for topic_to_suscribe in receive.get_topics() {
                    // for topic in &topics_copy {
                    //     if topic.clone().equals(receive.get_name()){
                    //         topic.add();
                    //     }
                    // }
                }
            }
        }); 

        topic_manager
    }

    pub fn get_publish_sender(&self) -> &Sender<Publish> {
        &self.publish_sender
    }
}
