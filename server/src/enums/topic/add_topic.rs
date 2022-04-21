use crate::enums::user_manager::user_manager_action::UserManagerAction;
use std::sync::mpsc::Sender;
pub struct AddTopic {
    client_id: String,
    sender: Sender<UserManagerAction>,
    qos: u8,
}

impl AddTopic {
    /// Instancia el struct para subscribir a un usuario.
    pub fn init(client_id: String, sender: Sender<UserManagerAction>, qos: u8) -> AddTopic {
        AddTopic {
            client_id,
            sender,
            qos,
        }
    }

    /// Obtiene el identificador del cliente que interviene en esta subscripcion.
    pub fn get_client_id(&self) -> String {
        self.client_id.to_owned()
    }

    /// Obtiene el sender del cliente que interviene en esta subscripcion.
    pub fn get_sender(&self) -> Sender<UserManagerAction> {
        self.sender.clone()
    }

    /// Obtiene el QoS de la subscripcion.
    pub fn get_qos(&self) -> u8 {
        self.qos
    }
}
