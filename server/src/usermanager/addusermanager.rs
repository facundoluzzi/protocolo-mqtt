use crate::stream::stream_handler::StreamType;
use std::sync::mpsc::Sender;


pub struct AddUserManager{
    client_id: String,
    sender_stream: Sender<StreamType>,
    clean_session: bool,
}

impl AddUserManager{

    pub fn init(client_id: String, sender_stream: Sender<StreamType>, clean_session: bool) -> AddUserManager{
        AddUserManager{
            client_id,
            sender_stream,
            clean_session,
        }
    }

    pub fn get_client_id(&self) -> String{
        self.client_id.to_string()
    }
    pub fn get_sender_stream(&self) -> Sender<StreamType>{
        self.sender_stream
    }
    pub fn get_clean_session(&self) -> bool{
        self.clean_session
    }

}
