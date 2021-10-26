use crate::flags::flags::Flags;

pub trait Payload {
    fn new(connect_flags: &Box<dyn Flags>, remaining_bytes: &[u8]) -> Box<dyn Payload>
    where
        Self: Sized;
}
