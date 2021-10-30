use crate::flags::connect_flags::ConnectFlags;
use crate::flags::trait_flags::Flags;
use crate::paquetes::Paquetes;
use crate::payload::default_payload::DefaultPayload;
use crate::payload::trait_payload::Payload;
use std::io::Write;
use std::net::TcpStream;

pub struct Default {
    payload: Box<dyn Payload>,
}

impl Paquetes for Default {
    fn save_remaining_length(&mut self, _bytes: &[u8]) -> Result<usize, String> {
        Ok(0)
    }

    fn get_remaining_length(&self) -> usize {
        0
    }

    fn init(_bytes: &[u8]) -> Box<dyn Paquetes> {
        Box::new(Default {
            payload: DefaultPayload::new(&ConnectFlags::new(&0x00u8), &[]),
        })
    }

    fn get_type(&self) -> String {
        "default".to_owned()
    }

    fn send_response(&self, mut stream: &TcpStream) {
        if let Err(msg_error) = stream.write("default message\n".as_bytes()) {
            println!("Error in sending response: {}", msg_error);
        }
    }

    fn get_payload(&self) -> &Box<dyn Payload> {
        &self.payload
    }
}
