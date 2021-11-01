use crate::flags::connect_flags::ConnectFlags;
use crate::paquetes::trait_paquetes::Paquetes;
use crate::payload::default_payload::DefaultPayload;
use std::io::Write;
use std::net::TcpStream;

pub struct Default {
    _payload: DefaultPayload,
}

impl Paquetes for Default {
    fn init(_bytes: &[u8]) -> Box<dyn Paquetes> {
        Box::new(Default {
            _payload: DefaultPayload::init(ConnectFlags::init(&0x00u8), &[]),
        })
    }

    fn get_type(&self) -> String {
        "default".to_owned()
    }

    fn send_response(&self, mut stream: &TcpStream) {
        if let Err(msg_error) = stream.write(b"default message\n") {
            println!("Error in sending response: {}", msg_error);
        }
    }
}
