/// Contiene el packet id para seleccionar el paquete y eliminarlo de los envíos automáticos
pub struct StopPublishToStream {
    packet_id: Vec<u8>,
}

impl StopPublishToStream {

    /// Constructor del struct
    pub fn init(packet_id: Vec<u8>) -> Self {
        StopPublishToStream { packet_id }
    }

    /// Devuelve el packet id
    pub fn get_packet_id(&self) -> Vec<u8> {
        self.packet_id.clone()
    }
}
