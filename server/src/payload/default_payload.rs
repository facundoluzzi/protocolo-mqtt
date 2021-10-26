use crate::flags::flags::Flags;
use crate::payload::payload::Payload;

use crate::utf8_parser::UTF8;

pub struct DefaultPayload {}

impl Payload for DefaultPayload {
    fn new(connect_flags: &Box<dyn Flags>, remaining_bytes: &[u8]) -> Box<dyn Payload> {
        Box::new(DefaultPayload {})
    }
}
