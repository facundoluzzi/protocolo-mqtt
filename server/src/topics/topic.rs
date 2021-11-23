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
        self.subscribers.insert(client_id, sender);
    }

    pub fn remove(&mut self, subscriber: String) {
        self.subscribers.remove(&subscriber);
    }

    pub fn publish_msg(&self, message: String) {
        for subscriber in self.subscribers.values() {
            if let Err(_msg) = subscriber.send(message.to_string()) {
                println!("Error al publicar el mensaje")
            };
        }
    }

    pub fn equals(&self, other_topic: String) -> bool {
        self.name == other_topic
    }
}

// #[cfg(test)]
// mod test_config_parser {
//     use std::sync::Arc;
// use std::sync::Mutex;
// use super::*;
//     use std::sync::mpsc;
//     use std::sync::mpsc::Receiver;
//     use std::thread;

//     #[test]
//     fn should_create_a_topic() {
//         let topic = Topic::new("deporte".to_string());
//         assert_eq!(topic.equals("deporte".to_string()), true);
//     }

//     #[test]
//     fn should_add_topic_and_publish_message() {
//         let mut topic = Topic::new("deporte".to_string());
//         let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
//         let messages: Vec<String> = Vec::new();
//         let data = Arc::new(Mutex::new(messages));
//         let data_for_thread = data.clone();
//         let thread = thread::spawn(move || {
//             let received_data = rx.recv();
//             let mut data = data_for_thread.lock().unwrap();
//             data.push(received_data.unwrap());
//         });
//         topic.add(tx);
//         topic.publish_msg("hola".to_string());
//         thread.join().unwrap();
//         let data = data.lock().unwrap();
//         assert_eq!(*data.get(0).unwrap(), "hola".to_string());        
//     }

//     #[test]
//     fn should_add_two_subscriber_and_public_same_message() {
//         let mut topic = Topic::new("deporte".to_string());
//         let (first_tx, first_rx): (Sender<String>, Receiver<String>) = mpsc::channel();
//         let (second_tx, second_rx): (Sender<String>, Receiver<String>) = mpsc::channel();

//         let messages: Vec<String> = Vec::new();
//         let data = Arc::new(Mutex::new(messages));
//         let data_for_first_thread = data.clone();
//         let data_for_second_thread = data.clone();

//         let first_thread = thread::spawn(move || {
//             let received_data = first_rx.recv();
//             let mut data = data_for_first_thread.lock().unwrap();
//             data.push(received_data.unwrap());
//         });

//         let second_thread = thread::spawn(move || {
//             let received_data = second_rx.recv();
//             let mut data = data_for_second_thread.lock().unwrap();
//             data.push(received_data.unwrap());
//         });

//         topic.add(first_tx);
//         topic.add(second_tx);
//         topic.publish_msg("hola".to_string());
        
//         first_thread.join().unwrap();
//         second_thread.join().unwrap();

//         let data = data.lock().unwrap();
//         assert_eq!(*data.get(0).unwrap(), "hola".to_string());
//         assert_eq!(*data.get(1).unwrap(), "hola".to_string());
//     }

//     #[test]
//     fn should_remove_the_subscriber_created() {
        
//     }
// }
