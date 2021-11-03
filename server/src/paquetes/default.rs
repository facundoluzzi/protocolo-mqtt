use crate::paquetes::trait_paquetes::Paquetes;
use std::io::Write;
use std::net::TcpStream;

pub struct Default {}

impl Paquetes for Default {
    fn init(_bytes: &[u8]) -> Box<dyn Paquetes> {
        Box::new(Default {})
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
