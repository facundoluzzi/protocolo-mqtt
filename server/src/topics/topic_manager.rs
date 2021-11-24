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
                        let topic_found = topics_copy
                            .iter()
                            .find(|topic| -> bool { topic.equals(publish_suscriber.get_topic()) });
                        let subscriber = publish_suscriber.get_sender().unwrap();
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
                };
            }
        });
        sender_to_return
    }

    pub fn get_sender(&self) -> Sender<PublisherSuscriber> {
        self.publisher_subscriber_sender.clone()
    }
}
