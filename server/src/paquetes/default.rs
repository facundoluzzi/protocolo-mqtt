use crate::flags::connect_flags::ConnectFlags;
use crate::flags::flags::Flags;
use crate::paquetes::trait_paquetes::Paquetes;
use crate::payload::default_payload::DefaultPayload;
use crate::payload::payload::Payload;
use std::io::Write;
use std::net::TcpStream;

pub struct Default {
    _payload: Box<dyn Payload>,
}

impl Paquetes for Default {
    fn init(_bytes: &[u8]) -> Box<dyn Paquetes> {
        Box::new(Default {
            _payload: DefaultPayload::new(&ConnectFlags::new(&0x00u8), &[]),
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
}
