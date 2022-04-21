use crate::enums::topic::add_topic::AddTopic;
use crate::enums::topic::publish_message::PublishMessage;
use crate::enums::topic::remove_topic::RemoveTopic;

/// Enum de tipos para desencadenar eventos a traves de un channel de TopicAction
/// por el que se maneja los packetes a los eventos referidos a un topico.
pub enum TopicAction {
    Add(AddTopic),
    Remove(RemoveTopic),
    Publish(PublishMessage),
}
