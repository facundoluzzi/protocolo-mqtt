use std::sync::mpsc::Sender;
use crate::topics::topic_actions::TopicAction;

pub type SenderTopicType = (TopicAction, String, Option<Sender<String>>);