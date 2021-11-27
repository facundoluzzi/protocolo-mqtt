use crate::helper::verify_wilcard;
use crate::helper::publisher_subscriber_code::PublisherSubscriberCode;
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
        let topics: Vec<Topic> = Vec::new();
        let sender_to_return = publisher_subscriber_sender.clone();
        let mut topic_manager = TopicManager {
            publisher_subscriber_sender,
            topics,
        };
        let topics_copy = topic_manager.topics.clone();
         thread::spawn(move || {
            for publish_suscriber in publisher_subscriber_receiver {
                match publish_suscriber.get_packet_type() {
                    PublisherSubscriberCode::Publisher => {
                        for topic in &topics_copy {
                            if topic.equals(publish_suscriber.get_topic()) {
                                topic.publish_msg(publish_suscriber.get_message());
                            }
                        }
                    }
                    PublisherSubscriberCode::Subscriber => {
                        let subscriber = publish_suscriber.get_sender().unwrap();
                        if let Some(wilcard) = verify_wilcard::verify_wilcard(publish_suscriber.get_topic()){
                            let topics_to_subscribe = topic_manager.get_topics(wilcard);
                            for mut t in topics_to_subscribe {
                                t.add(subscriber.clone(), publish_suscriber.get_client_id());
                            }
                        } else{
                            let topic_found = topics_copy
                            .iter()
                            .find(|topic| -> bool { topic.equals(publish_suscriber.get_topic()) });
                            if let Some(topic) = topic_found {
                                topic
                                    .clone()
                                    .add(subscriber, publish_suscriber.get_client_id());
                            } else {
                                let mut topic = Topic::new(publish_suscriber.get_topic());
                                topic.add(subscriber, publish_suscriber.get_client_id());
                                topic_manager.topics.push(topic);
                            }
                        }
                    }
                };
            }
        });
        sender_to_return
    }

    pub fn get_sender(&self) -> Sender<PublisherSuscriber> {
        self.publisher_subscriber_sender.clone()
    }

    pub fn get_topics(&self, wilcard: String) -> Vec<Topic> {
        let mut topics_to_return : Vec<Topic> = Vec::new();
        for topic in self.topics.clone() {
            let name = topic.get_name();
            let mut title = "".to_owned();
            let mut contains = false;
            for i in 0..name.len() {
                let b = name.as_bytes()[i] as char;
                title.push(b);
                if title == wilcard {
                    contains = true;
                }
            }
            if contains {
                topics_to_return.push(topic);
            }
        }
        topics_to_return
    }
}

