use crate::usermanager::user_manager_types::ChannelUserManager;
use std::sync::mpsc::Sender;

use crate::helper::publisher_subscriber_code::PublisherSubscriberCode;

pub struct PublisherSuscriber {
    client_id: String,
    code: PublisherSubscriberCode,
    topic: String,
    packet: Option<Vec<u8>>,
    sender_for_publish: Option<Sender<ChannelUserManager>>,
}

impl PublisherSuscriber {
    pub fn new(
        code: PublisherSubscriberCode,
        client_id: String,
        topic: String,
        sender: Option<Sender<ChannelUserManager>>,
        packet: Option<Vec<u8>>,
    ) -> PublisherSuscriber {
        PublisherSuscriber {
            topic,
            packet,
            code,
            sender_for_publish: sender,
            client_id,
        }
    }

    pub fn get_packet_type(&self) -> PublisherSubscriberCode {
        self.code
    }

    pub fn get_client_id(&self) -> String {
        self.client_id.to_owned()
    }

    pub fn get_topic(&self) -> String {
        self.topic.to_owned()
    }

    pub fn get_sender(&self) -> Option<Sender<ChannelUserManager>> {
        self.sender_for_publish.clone()
    }

    pub fn get_publish_packet(&self) -> Option<Vec<u8>> {
        match &self.packet {
            Some(bytes) => Some(bytes.clone()),
            None => None,
        }
    }
}
