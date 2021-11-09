use crate::topics::subscriber::Subscriber;

pub struct UserManager {
    users: Vec<Subscriber>,
}

impl UserManager {
    pub fn new() -> UserManager {
        UserManager {
            users :Vec::new()
        }
    }

    pub fn add(&self, user: Subscriber){
        self.users.push(user);
    }

    pub fn get_suscriber(&self, client_id: String) -> Option<Subscriber> {
        for subscriber in self.users {
            if subscriber.equals(client_id) {
                return Some(subscriber);
            }
        }
        None
    }

    pub fn delete_subscriber(&self, client_id: String)  {
        self.users.retain(|&x| x.equals(client_id))
    }
}