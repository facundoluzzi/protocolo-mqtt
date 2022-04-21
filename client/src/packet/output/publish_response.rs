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

    /// Obtiene la respuesta en forma de String correspondiente al estado del paquete Publish recibido
    pub fn get_response(&self) -> String {
        self.response.to_string()
    }

    /// Obtiene el topico del paquete Publish recibido
    pub fn get_topic(&self) -> String {
        self.topic.to_string()
    }

    /// Obtiene el qos del paquete Publish recibido
    pub fn get_qos(&self) -> u8 {
        self.qos
    }

    /// Obtiene el packet ID del paquete Publish recibido
    pub fn get_packet_identifier(self) -> Vec<u8> {
        match self.packet_identifier {
            Some(packet_id) => packet_id,
            None => vec![],
        }
    }
}
