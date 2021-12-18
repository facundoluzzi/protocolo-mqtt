use crate::enums::publisher_writter::publish_to_stream::PublishToStream;
use crate::enums::publisher_writter::reconnect_stream::ReconnectStream;
use crate::enums::publisher_writter::stop_publish_to_stream::StopPublishToStream;

/// Enum de tipos para desencadenar eventos a traves de un channel de en el publisher writer.
/// Por elevento pasamos solo la información necesitada en el publisher writer.
pub enum ChannelPublisherWriter {
    Publish(PublishToStream),
    Reconnect(ReconnectStream),
    Disconnect,
    StopToPublish(StopPublishToStream),
}
