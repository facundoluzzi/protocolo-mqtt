use crate::flags::flags::Flags;

pub trait Payload {
    fn get_username(&self) -> Option<String>;
    fn get_password(&self) -> Option<String>;
    fn new(connect_flags: &Box<dyn Flags>, remaining_bytes: &[u8]) -> Box<dyn Payload>
    where
        Self: Sized;
}
