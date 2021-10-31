use std::net::TcpStream;

pub trait Paquetes {
    fn init(bytes: &[u8]) -> Box<dyn Paquetes>
    where
        Self: Sized;
    fn get_type(&self) -> String;
    fn send_response(&self, stream: &TcpStream);
}
