use crate::flags::flags::Flags;
use crate::payload::payload::Payload;

pub struct DefaultPayload {}

impl Payload for DefaultPayload {
    fn new(_connect_flags: &Box<dyn Flags>, _remaining_bytes: &[u8]) -> Box<dyn Payload> {
        Box::new(DefaultPayload {})
    }
}
