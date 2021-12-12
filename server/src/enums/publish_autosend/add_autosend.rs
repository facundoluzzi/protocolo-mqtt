pub struct AddAutoSend {
    packet_identifier: Vec<u8>,
    publish: Vec<u8>,
}

impl AddAutoSend {
    pub fn init(packet_identifier: Vec<u8>, publish: Vec<u8>) -> AddAutoSend {
        AddAutoSend {
            packet_identifier,
            publish,
        }
    }

    pub fn get_packet_identifier(&self) -> Vec<u8> {
        self.packet_identifier.clone()
    }

    pub fn get_publish(&self) -> Vec<u8> {
        self.publish.clone()
    }
}
