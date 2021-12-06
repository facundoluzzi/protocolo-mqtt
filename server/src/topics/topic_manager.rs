use crate::helper::publisher_subscriber_code::PublisherSubscriberCode;
use crate::paquetes::publisher_suscriber::PublisherSuscriber;
use crate::topics::topic::Topic;
use crate::usermanager::user_manager_types::ChannelUserManager;
use crate::wildcard::wildcard_handler::Wildcard;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::topics::topic_actions::TopicAction::{AddTopic, PublishMessage, RemoveTopic};
use crate::topics::topic_types::SenderTopicType;

pub struct TopicManager {
    publisher_subscriber_sender: Sender<PublisherSuscriber>,
    topics: HashMap<String, Sender<SenderTopicType>>,
}

impl Clone for TopicManager {
    fn clone(&self) -> Self {
        let publisher_subscriber_sender = self.publisher_subscriber_sender.clone();
        Self {
            publisher_subscriber_sender,
            topics: self.topics.clone(),
        }
    }
}

impl TopicManager {
    pub fn init() -> Sender<PublisherSuscriber> {
        let (publisher_subscriber_sender, publisher_subscriber_receiver): (
            Sender<PublisherSuscriber>,
            Receiver<PublisherSuscriber>,
        ) = mpsc::channel();
        let sender_to_return = publisher_subscriber_sender.clone();

        let topics: HashMap<String, Sender<SenderTopicType>> = HashMap::new();
        let mut topic_manager = TopicManager {
            publisher_subscriber_sender,
            topics,
        };
        thread::spawn(move || {
            for publish_subscriber in publisher_subscriber_receiver {
                match publish_subscriber.get_packet_type() {
                    PublisherSubscriberCode::Publisher => {
                        topic_manager.publish_msg(
                            publish_subscriber.get_topic(),
                            publish_subscriber.get_message(),
                        );
                    }
                    PublisherSubscriberCode::Subscriber => {
                        let subscriber = publish_subscriber.get_sender().unwrap();
                        let topic_name = publish_subscriber.get_topic();
                        let client_id = publish_subscriber.get_client_id();
                        let wildcard = publish_subscriber.get_wildcard();

                        if let Some(wilcard) = wildcard {
                            topic_manager.subscribe_with_wilcard(
                                wilcard,
                                subscriber.clone(),
                                publish_subscriber.get_client_id(),
                            );
                        } else {
                            topic_manager.subscribe(
                                topic_name.to_owned(),
                                client_id.to_owned(),
                                subscriber,
                            );
                        }
                    }
                    PublisherSubscriberCode::Unsubscriber => {
                        let topic_name = publish_subscriber.get_topic();
                        let client_id = publish_subscriber.get_client_id();
                        topic_manager.unsubscribe(topic_name.to_owned(), client_id.to_owned());
                    }
                    PublisherSubscriberCode::UnsubscriberAll => {
                        let client_id = publish_subscriber.get_client_id();
                        topic_manager.unsubscribe_all(client_id.to_owned());
                    }
                }
            }
        });
        sender_to_return
    }

    fn publish_msg(&self, topic_name: String, message: String) {
        if let Some(topic_sender) = &self.topics.get(&topic_name) {
            topic_sender.send((PublishMessage, message, None)).unwrap();
        }
    }

    fn subscribe(
        &mut self,
        topic_name: String,
        client_id: String,
        sender_subscriber: Sender<ChannelUserManager>,
    ) {
        if let Some(topic_sender) = self.topics.get(&topic_name) {
            topic_sender
                .send((AddTopic, client_id, Some(sender_subscriber)))
                .unwrap();
        } else {
            let sender_topic = Topic::init(topic_name.to_owned());
            self.topics.insert(topic_name, sender_topic.clone());
            sender_topic
                .send((AddTopic, client_id, Some(sender_subscriber)))
                .unwrap();
        }
    }
    fn unsubscribe(&mut self, topic_name: String, client_id: String) {
        if let Some(topic_sender) = self.topics.get(&topic_name.to_owned()) {
            topic_sender
                .send((RemoveTopic, client_id.to_owned(), None))
                .unwrap();
        }
    }
    fn unsubscribe_all(&mut self, client_id: String) {
        for topic_sender in self.topics.values() {
            topic_sender
                .send((RemoveTopic, client_id.to_owned(), None))
                .unwrap();
        }
    }

    pub fn subscribe_with_wilcard(
        &self,
        wilcard: Wildcard,
        sender_subscribe: Sender<ChannelUserManager>,
        client_id: String,
    ) {
        for (topic_name, topic_sender) in &self.topics {
            if wilcard.verify_topic(topic_name.to_owned()) {
                topic_sender
                    .send((
                        AddTopic,
                        client_id.to_owned(),
                        Some(sender_subscribe.clone()),
                    ))
                    .unwrap();
            }
        }
    }
}
