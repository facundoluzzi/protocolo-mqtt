use crate::{
    usermanager::user_manager_types::ChannelUserManager, wildcard::wildcard_handler::Wildcard,
};
use std::sync::mpsc::Sender;

use crate::helper::publisher_subscriber_code::PublisherSubscriberCode;

pub struct PublisherSuscriber {
    client_id: String,
    code: PublisherSubscriberCode,
    topic: String,
    message: String,
    sender_for_publish: Option<Sender<ChannelUserManager>>,
    wildcard: Option<Wildcard>,
}

impl PublisherSuscriber {
    pub fn new(
        topic: String,
        message: String,
        code: PublisherSubscriberCode,
        sender: Option<Sender<ChannelUserManager>>,
        client_id: String,
        wildcard: Option<Wildcard>,
    ) -> PublisherSuscriber {
        PublisherSuscriber {
            topic,
            message,
            code,
            sender_for_publish: sender,
            client_id,
            wildcard,
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

    pub fn get_wildcard(&self) -> Option<Wildcard> {
        self.wildcard.clone()
    }

    pub fn get_message(&self) -> String {
        self.message.to_owned()
    }

    pub fn get_sender(&self) -> Option<Sender<ChannelUserManager>> {
        self.sender_for_publish.clone()
    }
}
