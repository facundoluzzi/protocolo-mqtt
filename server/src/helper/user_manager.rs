use std::{net::TcpStream, sync::mpsc::Sender};

use crate::topics::publisher_writer::PublisherWriter;

pub struct UserManager {
    users: Vec<PublisherWriter>,
}

impl Clone for UserManager {
    fn clone(&self) -> Self {
        UserManager {
            users: self.users.clone(),
        }
    }
}

impl UserManager {
    pub fn new() -> UserManager {
        UserManager { 
            users: Vec::new() 
        }
    }

    pub fn add(&mut self, client_id: String, stream: TcpStream) {
        let publisher_writer = PublisherWriter::init( stream, client_id);
        self.users.push(publisher_writer);
    }

    pub fn find_user(&self, client_id: String) -> Option<PublisherWriter>  {
        for publisher_writer in self.users.clone() {
            if publisher_writer.equals(client_id.to_string()) {
                return Some(publisher_writer);
            }
        }
        None
    }

    pub fn delete_user(&mut self, client_id: String) {
        self.users.retain(|x| !x.equals(client_id.to_string()))
    }

    pub fn get_sender(&self, client_id: String) -> Sender<String> {
        let publisher_writer = self.find_user(client_id).unwrap();
        publisher_writer.get_sender()
    }
}
