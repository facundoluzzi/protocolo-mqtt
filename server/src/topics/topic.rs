use std::sync::mpsc::Sender;

pub struct Topic {
    name: String,
    subscribers: Vec<Sender<String>>,
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
            subscribers: Vec::new(),
        }
    }

    pub fn add(mut self, sender: Sender<String>) {
        self.subscribers.push(sender);
    }

    pub fn remove(self, _subscriber: String) -> Result<String, String> {
        Ok("".to_string())
    }

    pub fn publish_msg(self, message: String) {
        for subscriber in self.subscribers {
            subscriber.send(message.to_string());
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
