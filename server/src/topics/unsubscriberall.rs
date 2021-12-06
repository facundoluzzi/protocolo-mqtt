use crate::topics::topic_actions::TopicAction::RemoveTopic;
use crate::topics::topic_types::SenderTopicType;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

pub struct UnsubscriberAll {
    client_id: String,
}

impl UnsubscriberAll {
    pub fn init(client_id: String) -> UnsubscriberAll {
        UnsubscriberAll { client_id }
    }

    pub fn unsubscribe_all(&mut self, topics: HashMap<String, Sender<SenderTopicType>>) {
        for topic_sender in topics.values() {
            topic_sender
                .send((
                    RemoveTopic,
                    Some(self.client_id.clone()),
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
}
