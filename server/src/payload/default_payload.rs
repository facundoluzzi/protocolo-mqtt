use crate::flags::connect_flags::ConnectFlags;

pub struct DefaultPayload {}

impl DefaultPayload {
    pub fn init(_connect_flags: ConnectFlags, _remaining_bytes: &[u8]) -> DefaultPayload {
       DefaultPayload {}
    }
    pub fn get_client_id(&self) -> String {
        "Default".to_owned()
    }
    pub fn get_username(&self) -> Option<&String> {
        None
    }
    pub fn get_password(&self) -> Option<&String> {
        None
    }
    pub fn get_will_topic(&self) -> Option<&String> {
        None
    }
    pub fn get_will_message(&self) -> Option<&String> {
        None
    }
}
