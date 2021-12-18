/// Contiene el client id y el packet identifier del paquete que va cortar su iteracion de publicacion con
/// DUP == 1.
pub struct StopPublish {
    client_id: String,
    packet_identifier: [u8; 2],
}

impl StopPublish {
    /// Constructor del struct
    pub fn init(client_id: String, packet_identifier: [u8; 2]) -> StopPublish {
        StopPublish {
            client_id,
            packet_identifier,
        }
    }

    /// Obtiene el client id
    pub fn get_client_id(&self) -> String {
        self.client_id.to_owned()
    }

    /// Obtiene el packet id
    pub fn get_packet_identifier(&self) -> [u8; 2] {
        self.packet_identifier
    }
}
