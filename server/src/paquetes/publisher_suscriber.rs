use std::net::TcpStream;

use crate::helper::publisher_subscriber_code::PublisherSubscriberCode;

pub struct PublisherSuscriber {
    code: PublisherSubscriberCode,
    topic: String,
    message: String,
    stream: TcpStream,
}

impl PublisherSuscriber {
    pub fn new(
        topic: String,
        message: String,
        code: PublisherSubscriberCode,
        stream: TcpStream,
    ) -> PublisherSuscriber {
        PublisherSuscriber {
            topic,
            message,
            code,
            stream,
        }
    }

    pub fn get_packet_type(&self) -> PublisherSubscriberCode {
        self.code
    }

    pub fn get_topic(&self) -> String {
        self.topic.to_owned()
    }

    pub fn get_stream(&self) -> TcpStream {
        self.stream.try_clone().unwrap()
    }

    pub fn get_message(&self) -> String {
        self.message.to_owned()
    }
}
