use crate::enums::topic::publish_message::PublishMessage;
use crate::enums::topic::topic_actions::TopicAction;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

pub struct Publisher {
    client_id: String,
    topic: String,
    publish: Vec<u8>,
    qos: u8,
    retained_message: bool,
    message: String,
}

impl Publisher {
    pub fn init(
        client_id: String,
        topic: String,
        publish: Vec<u8>,
        qos: u8,
        retained_message: bool,
        message: String,
    ) -> Publisher {
        Publisher {
            client_id,
            topic,
            publish,
            qos,
            retained_message,
            message,
        }
    }

    pub fn publish(&self, topics: HashMap<String, Sender<TopicAction>>) {
        let publish_packet = self.get_publish_packet();
        let topic_name = self.get_topic();
        if let Some(topic_sender) = &topics.get(&topic_name) {
            let publish = TopicAction::Publish(PublishMessage::init(
                publish_packet,
                self.qos,
                self.retained_message,
                self.message.to_string(),
            ));

            topic_sender.send(publish).unwrap();
        }
    }

    pub fn get_client_id(&self) -> String {
        self.client_id.to_string()
    }

    pub fn get_topic(&self) -> String {
        self.topic.to_string()
    }

    pub fn get_publish_packet(&self) -> Vec<u8> {
        self.publish.clone()
    }

    pub fn get_message(&self) -> String {
        self.message.to_string()
    }
}
