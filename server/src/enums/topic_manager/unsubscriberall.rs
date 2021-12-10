use crate::enums::topic::remove_topic::RemoveTopic;
use crate::enums::topic::topic_actions::TopicAction;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

pub struct UnsubscriberAll {
    client_id: String,
}

impl UnsubscriberAll {
    pub fn init(client_id: String) -> UnsubscriberAll {
        UnsubscriberAll { client_id }
    }

    pub fn unsubscribe_all(&mut self, topics: HashMap<String, Sender<TopicAction>>) {
        for topic_sender in topics.values() {
            let remove_topic = TopicAction::Remove(RemoveTopic::init(self.client_id.to_owned()));
            topic_sender.send(remove_topic).unwrap();
        }
    }

    pub fn get_client_id(&self) -> String {
        self.client_id.to_string()
    }
}
