/// Struct relacionado con una nueva desconeccion de un usuario.
pub struct DisconnectUserManager {
    client_id: String,
    disconnection_type: bool,
}

impl DisconnectUserManager {
    /// Instancia el struct de la desconeccion.
    pub fn init(client_id: String, disconnection_type: bool) -> DisconnectUserManager {
        DisconnectUserManager {
            client_id,
            disconnection_type,
        }
    }

    /// Obtiene el identificador del cliente que interviene en esta desconeccion.
    pub fn get_client_id(&self) -> String {
        self.client_id.to_string()
    }

    /// Obtiene el tipo de desconeccion que se va a ejecutar.
    /// True -> Ungracefully Desconnection (NO PAQUETE DISCONNECT)
    /// False -> Protocolar Disconnection (DISCONNECT EN PAQUETE)
    pub fn get_disconnection_type(&self) -> bool {
        self.disconnection_type
    }
}
