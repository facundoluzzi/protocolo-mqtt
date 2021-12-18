use crate::stream::stream_handler::StreamType;
use std::sync::mpsc::Sender;

pub struct AddUserManager {
    client_id: String,
    sender_stream: Sender<StreamType>,
    clean_session: bool,
    will_topic: Option<String>,
    will_message: Option<String>,
    will_qos: Option<u8>,
    will_retain_message: Option<bool>,
}

impl AddUserManager {
    pub fn init_with_will(
        client_id: String,
        sender_stream: Sender<StreamType>,
        clean_session: bool,
        will_topic: Option<String>,
        will_message: Option<String>,
        will_qos: Option<u8>,
        will_retained_message: Option<bool>,
    ) -> AddUserManager {
        AddUserManager {
            client_id,
            sender_stream,
            clean_session,
            will_topic: will_topic,
            will_message: will_message,
            will_qos: will_qos,
            will_retain_message: will_retained_message,
        }
    }

    pub fn init_without_will(
        client_id: String,
        sender_stream: Sender<StreamType>,
        clean_session: bool,
    ) -> Self {
        AddUserManager {
            client_id,
            sender_stream,
            clean_session,
            will_topic: None,
            will_message: None,
            will_qos: None,
            will_retain_message: None,
        }
    }

    pub fn get_client_id(&self) -> String {
        self.client_id.to_string()
    }
    pub fn get_sender_stream(&self) -> Sender<StreamType> {
        self.sender_stream.clone()
    }
    pub fn get_clean_session(&self) -> bool {
        self.clean_session
    }

    pub fn get_will_topic(&self) -> String {
        self.will_topic.to_owned().unwrap()
    }

    pub fn get_will_message(&self) -> Option<String> {
        self.will_message.to_owned()
    }

    pub fn get_will_qos(&self) -> u8 {
        self.will_qos.unwrap()
    }

    pub fn get_will_retain_message(&self) -> bool {
        self.will_retain_message.unwrap()
    }
}
