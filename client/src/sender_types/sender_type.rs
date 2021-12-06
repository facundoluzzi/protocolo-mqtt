use crate::sender_types::connect::Connect;
use crate::sender_types::publish::Publish;
use crate::sender_types::subscribe::Subscribe;

pub enum InterfaceSender {
    Connect(Connect),
    Publish(Publish),
    Subscribe(Subscribe),
}
