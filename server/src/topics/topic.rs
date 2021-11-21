use std::{collections::HashMap, sync::mpsc::Sender};

pub struct Topic {
    name: String,
    subscribers: HashMap<String, Sender<String>>,
}

impl Clone for Topic {
    fn clone(&self) -> Self {
        Topic {
            name: self.name.clone(),
            subscribers: self.subscribers.clone(),
        }
    }
}

impl Topic {
    pub fn new(name: String) -> Self {
        Topic {
            name,
            subscribers: HashMap::new(),
        }
    }

    pub fn add(&mut self, sender: Sender<String>, client_id: String) {
        self.subscribers.insert(client_id,sender);
    }

    pub fn remove(&mut self, subscriber: String) {
        self.subscribers.remove(&subscriber);
    }

    pub fn publish_msg(& self, message: String) {
        for subscriber in self.subscribers.values() {
            if let Err(_msg) = subscriber.send(message.to_string()){
                println!("Error al publicar el mensaje")
            };
        }
    }

    pub fn equals(&self, other_topic: String) -> bool {
        self.name == other_topic
    }

    pub fn get(self, name: String) -> Result<Self, String> {
        if self.name == name {
            Ok(Self {
                name: self.name,
                subscribers: self.subscribers,
            })
        } else {
            Err("".to_string())
        }
    }
}
