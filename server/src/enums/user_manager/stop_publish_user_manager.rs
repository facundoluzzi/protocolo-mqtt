pub struct StopPublish {
    client_id: String,
    packet_identifier: [u8; 2],
}

impl StopPublish {
    pub fn init(client_id: String, packet_identifier: [u8; 2]) -> StopPublish {
        StopPublish {
            client_id,
            packet_identifier,
        }
    }

    pub fn get_client_id(&self) -> String {
        self.client_id.to_owned()
    }

    pub fn get_packet_identifier(&self) -> [u8; 2] {
        self.packet_identifier
    }
}
