use crate::topics::topic_types::SenderTopicType;
use crate::topics::topic_actions::TopicAction::PublishMessage;
use std::sync::mpsc::Sender;
use std::collections::HashMap;

pub struct Publisher {
    client_id: String,
    topic: String,
    publish: Vec<u8>,
    qos: u8
}

impl Publisher {
    pub fn init(client_id: String, topic: String, publish: Vec<u8>, qos: u8) -> Publisher {
        Publisher {
            client_id,
            topic,
            publish,
            qos
        }
    }

    pub fn publish(&self, topics: HashMap<String, Sender<SenderTopicType>>) {
        let publish_packet = self.get_publish_packet();
        let topic_name = self.get_topic();

        if let Some(topic_sender) = &topics.get(&topic_name) {
            topic_sender
                .send((PublishMessage, None, Some(publish_packet), None, self.qos))
                .unwrap();
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
}
