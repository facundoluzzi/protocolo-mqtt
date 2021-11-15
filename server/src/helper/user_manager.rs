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
        UserManager { users: Vec::new() }
    }

    /// TODO: ACTUALIZAR
    ///
    /// # Ejemplo
    ///
    /// ```
    /// user_manager.add(publisher_writer)
    /// ```
    pub fn add(&mut self, client_id: String, stream: TcpStream) {
        let publisher_writer = PublisherWriter::init(stream, client_id);
        self.users.push(publisher_writer);
    }

    /// Itera los usuarios buscando un publisher que contenga el client_id solicitado, devuelve el usuario encontrado, sino devuelve None
    ///
    /// # Ejemplo
    ///
    /// ```
    /// user_manager.find_user("123".to_string())
    /// ```
    pub fn find_user(&self, client_id: String) -> Option<PublisherWriter> {
        for publisher_writer in self.users.clone() {
            if publisher_writer.equals(client_id.to_string()) {
                return Some(publisher_writer);
            }
        }
        None
    }

    /// Busca por client_id, y elimina el usuario
    ///
    /// # Ejemplo
    ///
    /// ```
    /// user_manager.delete_user("123".to_string())
    /// ```
    pub fn delete_user(&mut self, client_id: String) {
        // self.users.retain(|x| x.equals(client_id.to_string()))
    }

    pub fn get_sender(&self, client_id: String) -> Sender<String> {
        let publisher_writer = self.find_user(client_id).unwrap();
        publisher_writer.get_sender()
    }
}

// #[cfg(test)]
// mod tests {
//     use std::net::TcpStream;

//     use super::*;

//     #[test]
//     fn add_and_find_a_subscriber() {
//         let user_manager = UserManager::new();
//         let mut stream = TcpStream::connect("0.0.0.0:1883").unwrap();
//         let user = Subscriber::new("Pablito".to_owned(), stream);
//         user_manager.add(user);
//         assert_eq!(user, user_manager.find_user("Pablito".to_owned()));
//     }

//     fn add_and_find_then_delete_a_subscriber() {
//         let user_manager = UserManager::new();
//         let mut stream = TcpStream::connect("0.0.0.0:1883").unwrap();
//         let user = Subscriber::new("Pablito".to_owned(), stream);
//         user_manager.add(user);
//         assert_eq!(user, user_manager.find_user("Pablito".to_owned()));
//         user.delete_subscriber("Pablito".to_string());
//         assert_eq!(None, user_manager.find_user("Pablito".to_owned()));
//     }

//     fn find_a_non_existent_subscriber_and_get_none() {
//         let user_manager = UserManager::new();
//         assert_eq!(None, user_manager.find_user("Pablito".to_owned()));
//     }
// }
