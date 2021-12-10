use crate::sender_types;
use crate::sender_types::connack_response::ConnackResponse;
use crate::sender_types::connect::Connect;
use crate::sender_types::default_response::DefaultResponse;
use crate::sender_types::puback_response::PubackResponse;
use crate::sender_types::publish_response::PublishResponse;
use crate::sender_types::suback_response::SubackResponse;
use crate::sender_types::subscribe::Subscribe;
use crate::sender_types::unsuback_response::UnsubackResponse;
use crate::sender_types::unsubscribe::Unsubscribe;

pub enum InterfaceSender {
    Connect(Connect),
    Publish(sender_types::publish::Publish),
    Subscribe(Subscribe),
    Unsubscribe(Unsubscribe),
}

pub enum ClientSender {
    Connack(ConnackResponse),
    Puback(PubackResponse),
    Publish(PublishResponse),
    Suback(SubackResponse),
    Default(DefaultResponse),
    Unsuback(UnsubackResponse),
}
