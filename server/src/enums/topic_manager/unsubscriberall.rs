use crate::enums::topic::remove_topic::RemoveTopic;
use crate::enums::topic::topic_actions::TopicAction;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

/// Contiene el client id
pub struct UnsubscriberAll {
    client_id: String,
}

impl UnsubscriberAll {

    /// Constructor del struct
    pub fn init(client_id: String) -> UnsubscriberAll {
        UnsubscriberAll { client_id }
    }

    /// Recibe un hash map de topics y desuscribe al client id de todos los topicos que está suscripto
    pub fn unsubscribe_all(
        &mut self,
        topics: HashMap<String, Sender<TopicAction>>,
    ) -> HashMap<String, Sender<TopicAction>> {
        for topic_sender in topics.values() {
            let remove_topic = TopicAction::Remove(RemoveTopic::init(self.client_id.to_owned()));
            topic_sender.send(remove_topic).unwrap();
        }
        topics
    }

    /// Obtiene el client id
    pub fn get_client_id(&self) -> String {
        self.client_id.to_string()
    }
}
