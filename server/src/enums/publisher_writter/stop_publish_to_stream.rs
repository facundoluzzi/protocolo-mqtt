pub struct StopPublishToStream {
    packet_id: Vec<u8>,
}

impl StopPublishToStream {
    pub fn init(packet_id: Vec<u8>) -> Self {
        StopPublishToStream { packet_id }
    }

    pub fn get_packet_id(&self) -> Vec<u8> {
        self.packet_id.clone()
    }
}
