use crate::enums::topic::publish_message::PublishMessage;
use crate::enums::topic::topic_actions::TopicAction;
use crate::topic::topics::Topic;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

/// Contiene toda la información necesaria para publicar en cualquier caso.
pub struct Publisher {
    client_id: String,
    topic: String,
    publish: Vec<u8>,
    qos: u8,
    retained_message: bool,
    message: String,
}

impl Publisher {
    /// Constructor del struct
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

    fn send_publish(
        &self,
        publish: PublishMessage,
        topic_name: String,
        mut topics: HashMap<String, Sender<TopicAction>>,
    ) -> HashMap<String, Sender<TopicAction>> {
        match topics.get(&topic_name) {
            Some(topic_sender) => {
                println!("topic_name a: {}", topic_name);
                topic_sender.send(TopicAction::Publish(publish)).unwrap();
            }
            None => {
                println!("topic_name b: {}", topic_name);
                if publish.get_retained_message() {
                    println!("topic_name entro retained message true: {}", topic_name);
                    let sender_topic = Topic::init(self.topic.to_owned());
                    topics.insert(self.topic.to_owned(), sender_topic.clone());
                    sender_topic.send(TopicAction::Publish(publish)).unwrap();
                }
            }
        }
        topics
    }

    /// Selecciona el topico y publica. Si el topic no existe pero tiene retained message, crea el topico y publica
    pub fn publish(
        &self,
        topics: HashMap<String, Sender<TopicAction>>,
    ) -> HashMap<String, Sender<TopicAction>> {
        let publish_packet = self.get_publish_packet();
        let topic_name = self.get_topic();
        let publish = PublishMessage::init(
            publish_packet,
            self.qos,
            self.retained_message,
            self.message.to_string(),
        );
        self.send_publish(publish, topic_name, topics)
    }

    /// Obtiene el client id
    pub fn get_client_id(&self) -> String {
        self.client_id.to_string()
    }

    /// Obtiene el topic
    pub fn get_topic(&self) -> String {
        self.topic.to_string()
    }

    /// Obtiene el paquete publish completo
    pub fn get_publish_packet(&self) -> Vec<u8> {
        self.publish.clone()
    }

    /// Obtiene el mensaje
    pub fn get_message(&self) -> String {
        self.message.to_string()
    }
}
