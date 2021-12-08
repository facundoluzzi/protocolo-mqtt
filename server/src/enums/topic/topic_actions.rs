use crate::enums::topic::add_topic::AddTopic;
use crate::enums::topic::publish_message::PublishMessage;
use crate::enums::topic::remove_topic::RemoveTopic;
pub enum TopicAction {
    Add(AddTopic),
    Remove(RemoveTopic),
    Publish(PublishMessage),
}
