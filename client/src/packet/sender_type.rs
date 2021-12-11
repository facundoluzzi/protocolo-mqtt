use crate::packet::input::connect::Connect;
use crate::packet::input::publish::Publish;
use crate::packet::input::subscribe::Subscribe;
use crate::packet::output::connack_response::ConnackResponse;
use crate::packet::output::default_response::DefaultResponse;
use crate::packet::output::puback_response::PubackResponse;
use crate::packet::output::publish_response::PublishResponse;
use crate::packet::output::suback_response::SubackResponse;

pub enum InterfaceSender {
    Connect(Connect),
    Publish(Publish),
    Subscribe(Subscribe),
}

pub enum ClientSender {
    Connack(ConnackResponse),
    Puback(PubackResponse),
    Publish(PublishResponse),
    Suback(SubackResponse),
    Default(DefaultResponse),
}
