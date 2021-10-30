use crate::flags::trait_flags::Flags;

pub trait Payload {
    fn get_client_id(&self) -> String;
    fn get_username(&self) -> Option<&String>;
    fn get_password(&self) -> Option<&String>;
    fn get_will_topic(&self) -> Option<&String>;
    fn get_will_message(&self) -> Option<&String>;
    fn new(connect_flags: &Box<dyn Flags>, remaining_bytes: &[u8]) -> Box<dyn Payload>
    where
        Self: Sized;
}
