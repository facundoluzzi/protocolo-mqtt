use crate::enums::publisher_writter::publish_to_stream::PublishToStream;
use crate::enums::publisher_writter::reconnect_stream::ReconnectStream;
use crate::enums::publisher_writter::stop_publish_to_stream::StopPublishToStream;

pub enum ChannelPublisherWriter {
    Publish(PublishToStream),
    Reconnect(ReconnectStream),
    Disconnect,
    StopToPublish(StopPublishToStream),
}
