pub struct PublishMessageUserManager{
    client_id: String,
    message: Vec<u8>,
}

impl PublishMessageUserManager{

    pub fn init(client_id: String, message: Vec<u8>) -> PublishMessageUserManager{
        PublishMessageUserManager{
            client_id,
            message,
        }
    }

    pub fn get_client_id(&self) -> String{
        self.client_id.to_string()
    }
    pub fn get_message(&self) -> Vec<u8>{
        self.message.clone()
    }
}