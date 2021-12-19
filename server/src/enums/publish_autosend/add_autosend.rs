/// Struct usado para generar eventos a traves de un channel.
pub struct AddAutoSend {
    packet_identifier: Vec<u8>,
    publish: Vec<u8>,
}

impl AddAutoSend {
    /// Instancia el struct.
    pub fn init(packet_identifier: Vec<u8>, publish: Vec<u8>) -> AddAutoSend {
        AddAutoSend {
            packet_identifier,
            publish,
        }
    }
    /// Devuelve el packet identifier.
    pub fn get_packet_identifier(&self) -> Vec<u8> {
        self.packet_identifier.clone()
    }
    /// Devuelve el packete como un vector de bytes.
    pub fn get_publish(&self) -> Vec<u8> {
        self.publish.clone()
    }
}
