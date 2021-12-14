use crate::packet::input::connect::Connect;
use crate::packet::input::disconnect::Disconnect;
use crate::packet::input::publish::Publish;
use crate::packet::input::subscribe::Subscribe;
use crate::packet::input::unsubscribe::Unsubscribe;
use crate::packet::output::connack_response::ConnackResponse;
use crate::packet::output::connect_error_response::ConnectErrorResponse;
use crate::packet::output::default_response::DefaultResponse;
use crate::packet::output::disconnect_response::DisconnectResponse;
use crate::packet::output::puback_response::PubackResponse;
use crate::packet::output::publish_response::PublishResponse;
use crate::packet::output::suback_response::SubackResponse;
use crate::packet::output::unsuback_response::UnsubackResponse;

pub enum InterfaceSender {
    Connect(Connect),
    Publish(Publish),
    Subscribe(Subscribe),
    Unsubscribe(Unsubscribe),
    Disconnect(Disconnect),
}

pub enum ClientSender {
    Connack(ConnackResponse),
    Puback(PubackResponse),
    Publish(PublishResponse),
    Suback(SubackResponse),
    Default(DefaultResponse),
    Unsuback(UnsubackResponse),
    Disconnect(DisconnectResponse),
    ConnectError(ConnectErrorResponse),
}
