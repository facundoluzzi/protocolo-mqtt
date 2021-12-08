use crate::topics::publisher::Publisher;
use crate::topics::subscriber::Subscriber;
use crate::topics::topic_actions::TopicAction;
use crate::topics::unsubscriber::Unsubscriber;
use crate::topics::unsubscriberall::UnsubscriberAll;
use crate::usermanager::user_manager_action::UserManagerAction;
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
