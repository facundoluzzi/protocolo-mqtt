use std::sync::mpsc::Sender;

/// Struct utilizado para validar si un client id ya existe en el user manager
pub struct ValidClientIdUserManager {
    client_id: String,
    sender: Sender<bool>,
}

impl ValidClientIdUserManager {
    /// Constructor del struct
    pub fn init(client_id: String, sender: Sender<bool>) -> Self {
        ValidClientIdUserManager { client_id, sender }
    }

    /// obtiene el client id
    pub fn get_client_id(&self) -> String {
        self.client_id.to_string()
    }

    /// obtiene el sender para responder si es valido o no
    pub fn get_sender(&self) -> Sender<bool> {
        self.sender.clone()
    }
}
