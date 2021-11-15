use std::sync::mpsc::Sender;

use crate::helper::publisher_subscriber_code::PublisherSubscriberCode;

pub struct PublisherSuscriber {
    code: PublisherSubscriberCode,
    topic: String,
    message: String,
    sender: Option<Sender<String>>,
}

impl PublisherSuscriber {
    pub fn new(
        topic: String,
        message: String,
        code: PublisherSubscriberCode,
        sender: Option<Sender<String>>,
    ) -> PublisherSuscriber {
        PublisherSuscriber {
            topic,
            message,
            code,
            sender,
        }
    }

    pub fn get_packet_type(&self) -> PublisherSubscriberCode {
        self.code
    }

    pub fn get_topic(&self) -> String {
        self.topic.to_owned()
    }

    pub fn get_message(&self) -> String {
        self.message.to_owned()
    }

    pub fn get_sender(&self) -> Option<Sender<String>> {
        self.sender.clone()
    }
}
