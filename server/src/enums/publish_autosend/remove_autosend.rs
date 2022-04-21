/// Struct usado para generar eventos a traves de un channel.
pub struct RemoveAutoSend {
    packet_identifier: Vec<u8>,
}

impl RemoveAutoSend {
    /// Instancia el struct.
    pub fn init(packet_identifier: Vec<u8>) -> RemoveAutoSend {
        RemoveAutoSend { packet_identifier }
    }

    /// Devuelve el packet identifier.
    pub fn get_packet_identifier(&self) -> Vec<u8> {
        self.packet_identifier.clone()
    }
}
