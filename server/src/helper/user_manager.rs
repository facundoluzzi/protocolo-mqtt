use crate::topics::publisher_writer::PublisherWriter;
use std::{collections::HashMap, net::TcpStream, sync::mpsc::Sender};
pub struct UserManager {
    users: HashMap<String, (PublisherWriter, bool)>,
}

impl Clone for UserManager {
    fn clone(&self) -> Self {
        UserManager {
            users: self.users.clone(),
        }
    }
}

impl Default for UserManager {
    fn default() -> Self {
        Self::new()
    }
}

impl UserManager {
    pub fn new() -> UserManager {
        UserManager {
            users: HashMap::new(),
        }
    }

    pub fn add(&mut self, client_id: String, stream: TcpStream, clean_session: bool) {
        let publisher_writer = PublisherWriter::init(stream, client_id.to_owned());
        self.users
            .insert(client_id, (publisher_writer, clean_session));
    }

    pub fn find_user(&self, client_id: String) -> Option<PublisherWriter> {
        if let Some(publisher_writer) = self.users.get(&client_id) {
            Some(publisher_writer.0.clone())
        } else {
            None
        }
    }

    pub fn delete_user(&mut self, client_id: String) {
        if self.users.remove(&client_id).is_none() {
            println!("Unexpected error");
        }
    }

    pub fn get_sender(&self, client_id: String) -> Sender<String> {
        let publisher_writer = self.find_user(client_id).unwrap();
        publisher_writer.get_sender()
    }
}
