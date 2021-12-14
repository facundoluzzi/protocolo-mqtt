pub struct RemoveAutoSend {
    packet_identifier: Vec<u8>,
}

impl RemoveAutoSend {
    pub fn init(packet_identifier: Vec<u8>) -> RemoveAutoSend {
        RemoveAutoSend { packet_identifier }
    }

    pub fn get_packet_identifier(&self) -> Vec<u8> {
        self.packet_identifier.clone()
    }
}
