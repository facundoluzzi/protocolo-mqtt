pub struct DisconnectUserManager {
    client_id: String,
    disconnection_type: bool,
}

impl DisconnectUserManager {
    pub fn init(client_id: String, disconnection_type: bool) -> DisconnectUserManager {
        DisconnectUserManager {
            client_id,
            disconnection_type,
        }
    }

    pub fn get_client_id(&self) -> String {
        self.client_id.to_string()
    }

    pub fn get_disconnection_type(&self) -> bool {
        self.disconnection_type
    }
}
