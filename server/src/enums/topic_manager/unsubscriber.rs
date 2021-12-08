use crate::enums::topic::remove_topic::RemoveTopic;
use crate::enums::topic::topic_actions::TopicAction;
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

    pub fn unsubscribe(&mut self, topics: HashMap<String, Sender<TopicAction>>) {
        if let Some(topic_sender) = topics.get(&self.topic.to_owned()) {
            let remove_topic = TopicAction::Remove(RemoveTopic::init(self.client_id.to_owned()));
            topic_sender.send(remove_topic).unwrap();
        }
    }

    pub fn get_client_id(&self) -> String {
        self.client_id.to_string()
    }

    pub fn get_topic(&self) -> String {
        self.topic.to_string()
    }
}
