pub struct DisconnectUserManager {
    client_id: String,
}

impl DisconnectUserManager {
    pub fn init(client_id: String) -> DisconnectUserManager {
        DisconnectUserManager { client_id }
    }

    pub fn get_client_id(&self) -> String {
        self.client_id.to_string()
    }
}
