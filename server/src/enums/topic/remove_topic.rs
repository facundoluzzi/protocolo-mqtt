pub struct RemoveTopic {
    client_id: String,
}

impl RemoveTopic {
    pub fn init(client_id: String) -> RemoveTopic {
        RemoveTopic { client_id }
    }

    pub fn get_client_id(&self) -> String {
        self.client_id.to_owned()
    }
}
