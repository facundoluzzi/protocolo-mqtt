use crate::topics::subscriber::Subscriber;

pub struct UserManager {
    users: Vec<Subscriber>,
}

impl Clone for UserManager {
    fn clone(&self) -> Self {
        UserManager {
            users: self.users.clone()
        }
    }
}

impl UserManager {
    pub fn new() -> UserManager {
        UserManager {
            users: Vec::new()
        }
    }

    pub fn add(&mut self, user: Subscriber){
        self.users.push(user);
    }

    pub fn find_user(&self, client_id: String) -> Option<Subscriber> {
        for subscriber in self.users.clone() {
            if subscriber.equals(client_id.to_string()) {
                return Some(subscriber);
            }
        }
        None
    }

    pub fn delete_subscriber(&mut self, client_id: String)  {
        self.users.retain(|x| x.equals(client_id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::net::TcpStream;

    use super::*;

    #[test]
    fn add_and_find_a_subscriber() {
        let user_manager = UserManager::new();
        let mut stream = TcpStream::connect("0.0.0.0:1883").unwrap();
        let user = Subscriber::new("Pablito".to_owned(), stream);
        user_manager.add(user);
        assert_eq!(user, user_manager.find_user("Pablito".to_owned()));
    }

    fn add_and_find_then_delete_a_subscriber() {
        let user_manager = UserManager::new();
        let mut stream = TcpStream::connect("0.0.0.0:1883").unwrap();
        let user = Subscriber::new("Pablito".to_owned(), stream);
        user_manager.add(user);
        assert_eq!(user, user_manager.find_user("Pablito".to_owned()));
        user.delete_subscriber("Pablito".to_string());
        assert_eq!(None, user_manager.find_user("Pablito".to_owned()));
    }

    fn find_a_non_existent_subscriber_and_get_none() {
        let user_manager = UserManager::new();
        assert_eq!(None, user_manager.find_user("Pablito".to_owned()));
    }
}