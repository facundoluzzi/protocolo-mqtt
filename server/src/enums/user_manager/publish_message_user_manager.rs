/// Struct relacionado con una publicacion de un mensaje en el PublisherWriter para que llege al cliente.
/// Intermedia entre el topico y la punta a la que se conecta el usuario.
pub struct PublishMessageUserManager {
    client_id: String,
    message: Vec<u8>,
}

impl PublishMessageUserManager {
    /// Instancia el struct que desencadena el evento de publicar en el Stream desde el PublisherWriter correspondiente al usuario.
    pub fn init(client_id: String, message: Vec<u8>) -> PublishMessageUserManager {
        PublishMessageUserManager { client_id, message }
    }

    /// Obtiene el identificador del cliente al que se le va hacer el envio.
    pub fn get_client_id(&self) -> String {
        self.client_id.to_string()
    }

    /// Obtiene el mensaje que se va a enviar.
    pub fn get_message(&self) -> Vec<u8> {
        self.message.clone()
    }
}
