use crate::helper::publisher_subscriber_code::PublisherSubscriberCode;
use crate::topics::subscriber::Subscriber;

pub struct PublisherSuscriber {
    code: PublisherSubscriberCode,
    topic: String,
    message: String,
    subscriber: Option<Subscriber>,
}

impl PublisherSuscriber {
    pub fn new(
        topic: String,
        message: String,
        code: PublisherSubscriberCode,
        subscriber: Option<Subscriber>,
    ) -> PublisherSuscriber {
        PublisherSuscriber {
            topic,
            message,
            code,
            subscriber,
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

    pub fn get_subscriber(&self) -> Option<Subscriber> {
        self.subscriber.clone()
    }
}
