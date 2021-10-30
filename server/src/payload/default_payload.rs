use crate::flags::trait_flags::Flags;
use crate::payload::trait_payload::Payload;

pub struct DefaultPayload {}

impl Payload for DefaultPayload {
    fn init(_connect_flags: &Box<dyn Flags>, _remaining_bytes: &[u8]) -> Box<dyn Payload> {
        Box::new(DefaultPayload {})
    }
    fn get_client_id(&self) -> String {
        "Default".to_owned()
    }
    fn get_username(&self) -> Option<&String> {
        None
    }
    fn get_password(&self) -> Option<&String> {
        None
    }
    fn get_will_topic(&self) -> Option<&String> {
        None
    }
    fn get_will_message(&self) -> Option<&String> {
        None
    }
}
