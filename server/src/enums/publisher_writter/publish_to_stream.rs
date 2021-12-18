/// Contiene el mensaje que recibe el publisher writer para publicar
pub struct PublishToStream {
    message: Vec<u8>,
}

impl PublishToStream {
    /// Constructor del PublishToStream
    pub fn init(message: Vec<u8>) -> Self {
        PublishToStream { message }
    }

    /// Devuelve el mensaje
    pub fn get_message(&self) -> Vec<u8> {
        self.message.clone()
    }
}
