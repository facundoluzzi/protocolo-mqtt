use crate::enums::topic::add_topic::AddTopic;
use crate::enums::topic::topic_actions::TopicAction;
use crate::topic::topics::Topic;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

use crate::enums::user_manager::user_manager_action::UserManagerAction;
use crate::wildcard::wildcard_handler::Wildcard;

/// Contiene información necesaria para suscribir de forma correcta
pub struct Subscriber {
    client_id: String,
    topic: String,
    sender_user_manager: Sender<UserManagerAction>,
    wildcard: Option<Wildcard>,
    qos: u8,
}

impl Subscriber {
    /// Constructor del struct
    pub fn init(
        client_id: String,
        topic: String,
        sender_user_manager: Sender<UserManagerAction>,
        wildcard: Option<Wildcard>,
        qos: u8,
    ) -> Subscriber {
        Subscriber {
            client_id,
            topic,
            sender_user_manager,
            wildcard,
            qos,
        }
    }

    /// Recibe un hash map de topics y subscribe
    pub fn subscribe(
        &mut self,
        topics: HashMap<String, Sender<TopicAction>>,
    ) -> HashMap<String, Sender<TopicAction>> {
        match &self.wildcard {
            Some(wildcard) => self.subscribe_with_wildcard(wildcard.clone(), topics),
            None => self.subscribe_without_wilcard(topics),
        }
    }

    fn subscribe_without_wilcard(
        &mut self,
        mut topics: HashMap<String, Sender<TopicAction>>,
    ) -> HashMap<String, Sender<TopicAction>> {
        match topics.get(&self.topic) {
            Some(topic_sender) => {
                let add_topic = TopicAction::Add(AddTopic::init(
                    self.client_id.to_owned(),
                    self.sender_user_manager.clone(),
                    self.qos,
                ));
                topic_sender.send(add_topic).unwrap();
            }
            None => {
                let sender_topic = Topic::init(self.topic.to_owned());
                topics.insert(self.topic.to_owned(), sender_topic.clone());
                let add_topic = TopicAction::Add(AddTopic::init(
                    self.client_id.to_owned(),
                    self.sender_user_manager.clone(),
                    self.qos,
                ));
                sender_topic.send(add_topic).unwrap();
            }
        }

        topics
    }

    /// Recibe una wildcard y un hash map de topics y subscribe, si la wildcard matchea con un topico, lo subscribe.
    pub fn subscribe_with_wildcard(
        &self,
        wildcard: Wildcard,
        topics: HashMap<String, Sender<TopicAction>>,
    ) -> HashMap<String, Sender<TopicAction>> {
        for (topic_name, sender_topic) in &topics {
            if wildcard.verify_topic(topic_name.to_owned()) {
                let add_topic = TopicAction::Add(AddTopic::init(
                    self.client_id.to_owned(),
                    self.sender_user_manager.clone(),
                    self.qos,
                ));
                sender_topic.send(add_topic).unwrap();
            }
        }
        topics
    }

    /// Obtiene el client id
    pub fn get_client_id(&self) -> String {
        self.client_id.to_string()
    }

    /// Obtiene el topic
    pub fn get_topic(&self) -> String {
        self.topic.to_string()
    }

    /// Obtiene el sender de user manager
    pub fn get_sender_user_manager(&self) -> Sender<UserManagerAction> {
        self.sender_user_manager.clone()
    }

    /// Obtiene la wildcard
    pub fn get_wildcard(&self) -> Option<Wildcard> {
        self.wildcard.as_ref().cloned()
    }

    /// Obtiene el qos
    pub fn get_qos(&self) -> u8 {
        self.qos
    }
}
