use crate::enums::topic::topic_actions::TopicAction::RemoveTopic;
use crate::types::topic_types::SenderTopicType;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

pub struct Unsubscriber {
    client_id: String,
    topic: String,
}

impl Unsubscriber {
    pub fn init(client_id: String, topic: String) -> Unsubscriber {
        Unsubscriber { client_id, topic }
    }

    pub fn unsubscribe(&mut self, topics: HashMap<String, Sender<SenderTopicType>>) {
        if let Some(topic_sender) = topics.get(&self.topic.to_owned()) {
            topic_sender
                .send((
                    RemoveTopic,
                    Some(self.client_id.to_owned()),
                    None,
                    None,
                    0,
                    None,
                ))
                .unwrap();
        }
    }

    pub fn get_client_id(&self) -> String {
        self.client_id.to_string()
    }

    pub fn get_topic(&self) -> String {
        self.topic.to_string()
    }
}
