use crate::paquetes::publisher_suscriber::PublisherSuscriber;
use crate::topics::topic::Topic;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub struct TopicManager {
    publisher_subscriber_sender: Sender<PublisherSuscriber>,
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
            Sender<PublisherSuscriber>,
            Receiver<PublisherSuscriber>,
        ) = mpsc::channel();
        let topics: Vec<Topic> = Vec::new();

        let topic_manager = TopicManager {
            publisher_subscriber_sender,
            topics,
        };
        let mut topics_copy = topic_manager.topics.clone();

        thread::spawn(move || {
            for publish_suscriber in publisher_subscriber_receiver {
                // hay que crear un struct PublisherSubscriber que tenga el tipo, recibimos un struct de ese tipo acá.
                // Dependiendo de que haga, lo podemos mandar a dos threads diferentes o no. Pero nos puede servir para bloquear
                // los publishers mientras hayan subscripciones en proceso o lo opuesto.
                // // topics_copy.push(Topic::new(publish_suscriber));

                let packet_type = publish_suscriber.get_packet_type();
                if packet_type.eq("Publish") {
                    for topic in &topics_copy {
                        if topic.clone().equals(publish_suscriber.get_topic()){
                            topic.clone().publish_msg(publish_suscriber.get_message());
                        }
                    }
                }
                else {
                    for topic in &topics_copy {
                        if topic.clone().equals(publish_suscriber.get_topic()){
                            topic.clone().add("UnSuscriptor!!1".to_owned());
                        }
                    }
                }

            }
        });

        topic_manager
    }

    pub fn get_sender(&self) -> Sender<PublisherSuscriber> {
        self.publisher_subscriber_sender.clone()
    }
}
