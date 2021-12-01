use crate::topics::topic_actions::TopicAction;
use std::sync::mpsc::Sender;

pub type SenderTopicType = (TopicAction, String, Option<Sender<String>>);
