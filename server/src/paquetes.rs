use crate::payload::trait_payload::Payload;
use std::net::TcpStream;

pub trait Paquetes {
    fn save_remaining_length(&mut self, bytes: &[u8]) -> Result<usize, String>;
    fn get_remaining_length(&self) -> usize;
    fn init(bytes: &[u8]) -> Box<dyn Paquetes>
    where
        Self: Sized;
    fn get_type(&self) -> String;
    fn send_response(&self, stream: &TcpStream);
    fn get_payload(&self) -> &Box<dyn Payload>;
}
