/// Struct para desencadenar un evento de publicacion de un topico.
pub struct PublishMessage {
    all_bytes: Vec<u8>,
    qos: u8,
    retained_message: bool,
    message: String,
}

impl Clone for PublishMessage {
    fn clone(&self) -> Self {
        Self {
            all_bytes: self.all_bytes.clone(),
            qos: self.qos,
            retained_message: self.retained_message,
            message: self.message.to_string(),
        }
    }
}

impl PublishMessage {
    /// Instancia el struct relacionado con la publicacion.
    pub fn init(
        all_bytes: Vec<u8>,
        qos: u8,
        retained_message: bool,
        message: String,
    ) -> PublishMessage {
        PublishMessage {
            all_bytes,
            qos,
            retained_message,
            message,
        }
    }

    /// Obtiene el paquete como un vector de bytes.
    pub fn get_packet(&self) -> Vec<u8> {
        self.all_bytes.clone()
    }

    /// Obtiene el mensaje como un string.
    pub fn get_message(&self) -> String {
        self.message.to_string()
    }

    /// Obtiene el QoS.
    pub fn get_qos(&self) -> u8 {
        self.qos
    }

    /// Obtiene el flag de si es o no un retained massage.
    pub fn get_retained_message(&self) -> bool {
        self.retained_message
    }
}
