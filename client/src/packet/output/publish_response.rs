pub struct PublishResponse {
    response: String,
    topic: String,
    qos: u8,
    packet_identifier: Option<Vec<u8>>,
}

impl PublishResponse {
    pub fn init(
        topic: String,
        response: String,
        qos: u8,
        packet_identifier: Option<Vec<u8>>,
    ) -> PublishResponse {
        PublishResponse {
            response,
            topic,
            qos,
            packet_identifier,
        }
    }

    pub fn get_response(&self) -> String {
        self.response.to_string()
    }

    pub fn get_topic(&self) -> String {
        self.topic.to_string()
    }

    pub fn get_qos(&self) -> u8 {
        self.qos
    }

    pub fn get_packet_identifier(self) -> Vec<u8> {
        match self.packet_identifier {
            Some(packet_id) => packet_id,
            None => vec![],
        }
    }
}
