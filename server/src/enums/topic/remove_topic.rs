/// Struct relacionado con una desubscripcion a un topico.
pub struct RemoveTopic {
    client_id: String,
}

impl RemoveTopic {
    /// Instancia el struct para la desubscripcion.
    pub fn init(client_id: String) -> RemoveTopic {
        RemoveTopic { client_id }
    }

    /// Obtiene el identificador del cliente que interviene en esta desubcripcion.
    pub fn get_client_id(&self) -> String {
        self.client_id.to_owned()
    }
}
