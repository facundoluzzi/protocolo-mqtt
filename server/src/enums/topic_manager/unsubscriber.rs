use crate::enums::topic::remove_topic::RemoveTopic;
use crate::enums::topic::topic_actions::TopicAction;
use crate::wildcard::wildcard_handler::Wildcard;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

pub struct Unsubscriber {
    client_id: String,
    topic: String,
    wildcard: Option<Wildcard>,
}

impl Unsubscriber {
    pub fn init(client_id: String, topic: String, wildcard: Option<Wildcard>) -> Unsubscriber {
        Unsubscriber {
            client_id,
            topic,
            wildcard,
        }
    }

    pub fn unsubscribe(
        &mut self,
        topics: HashMap<String, Sender<TopicAction>>,
    ) -> HashMap<String, Sender<TopicAction>> {
        match &self.wildcard {
            Some(wildcard) => self.unsubscribe_with_wilcard(wildcard.clone(), topics),
            None => self.unsubscribe_without_wilcard(topics),
        }
        // if let Some(topic_sender) = topics.get(&self.topic.to_owned()) {
        //     let remove_topic = TopicAction::Remove(RemoveTopic::init(self.client_id.to_owned()));
        //     topic_sender.send(remove_topic).unwrap();
        // }
        // topics
    }

    fn unsubscribe_with_wilcard(
        &self,
        wildcard: Wildcard,
        topics: HashMap<String, Sender<TopicAction>>,
    ) -> HashMap<String, Sender<TopicAction>> {
        if let Some(topic_sender) = topics.get(&self.topic.to_owned()) {
            let remove_topic = TopicAction::Remove(RemoveTopic::init(self.client_id.to_owned()));
            topic_sender.send(remove_topic).unwrap();
        }

        for (topic_name, topic_sender) in &topics {
            if wildcard.verify_topic(topic_name.to_owned()) {
                let remove_topic =
                    TopicAction::Remove(RemoveTopic::init(self.client_id.to_owned()));
                topic_sender.send(remove_topic).unwrap();
            }
        }
        topics
    }

    fn unsubscribe_without_wilcard(
        &self,
        topics: HashMap<String, Sender<TopicAction>>,
    ) -> HashMap<String, Sender<TopicAction>> {
        if let Some(topic_sender) = topics.get(&self.topic.to_owned()) {
            let remove_topic = TopicAction::Remove(RemoveTopic::init(self.client_id.to_owned()));
            topic_sender.send(remove_topic).unwrap();
        }
        topics
    }

    pub fn get_client_id(&self) -> String {
        self.client_id.to_string()
    }

    pub fn get_topic(&self) -> String {
        self.topic.to_string()
    }
}
