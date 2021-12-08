use crate::enums::topic::publisher::Publisher;
use crate::enums::topic::subscriber::Subscriber;
use crate::enums::topic::topic_actions::TopicAction;
use crate::enums::topic::unsubscriber::Unsubscriber;
use crate::enums::topic::unsubscriberall::UnsubscriberAll;
use crate::enums::user_manager::user_manager_action::UserManagerAction;
use std::sync::mpsc::Sender;

pub type SenderTopicType = (
    TopicAction,
    Option<String>,
    Option<Vec<u8>>,
    Option<Sender<UserManagerAction>>,
    u8,
    Option<bool>,
);

pub enum TypeTopicManager {
    Publisher(Publisher),
    Subscriber(Subscriber),
    Unsubscriber(Unsubscriber),
    UnsubscriberAll(UnsubscriberAll),
}
