use crate::topics::topic_actions::TopicAction;
use crate::usermanager::user_manager_types::ChannelUserManager;
use std::sync::mpsc::Sender;

pub type SenderTopicType = (
    TopicAction,
    Option<String>,
    Option<Vec<u8>>,
    Option<Sender<ChannelUserManager>>,
);
