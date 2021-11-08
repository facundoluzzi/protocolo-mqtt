use crate::helper::publisher_subscriber_code::PublisherSubscriberCode;

pub struct PublisherSuscriber {
    code: PublisherSubscriberCode,
    topic: String,
    message: String,
}


impl PublisherSuscriber {
    pub fn new(topic: String, message: String, code: PublisherSubscriberCode) -> PublisherSuscriber{
        PublisherSuscriber{ topic, message, code }
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

}