use crate::stream::stream_handler::StreamAction::WriteStream;
use crate::stream::stream_handler::StreamType;
use std::sync::mpsc::{self, Receiver, Sender};

pub struct PublisherWriter {
    sender: Sender<String>,
    socket: Option<Sender<StreamType>>,
    client_id: String,
    queue: Vec<String>,
}

impl Clone for PublisherWriter {
    fn clone(&self) -> PublisherWriter {
        PublisherWriter {
            sender: self.sender.clone(),
            socket: Some(self.socket.as_ref().unwrap().clone()),
            client_id: self.client_id.to_string(),
            queue: self.queue.clone(),
        }
    }
}

impl PublisherWriter {
    pub fn init(socket: Sender<StreamType>, client_id: String) -> PublisherWriter {
        let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();
        // crear un receiver
        let mut publisher = PublisherWriter {
            sender,
            socket: Some(socket),
            client_id,
            queue: Vec::new(),
        };
        let publisher_cloned = publisher.clone();
        std::thread::spawn(move || {
            for receive in receiver {
                publisher.publish_message(receive);
            }
        });
        publisher_cloned
    }

    pub fn get_sender(&self) -> Sender<String> {
        self.sender.clone()
    }

    pub fn publish_message(&mut self, receive: String) {
        if let Some(socket) = &self.socket {
            let new_vec: Vec<u8> = receive.as_bytes().to_vec();
            socket.send((WriteStream, Some(new_vec), None)).unwrap();
        } else {
            self.queue.push(receive);
        }
    }

    pub fn reconnect(&mut self, stream: Sender<StreamType>) {
        self.socket = Some(stream);
        for message in self.queue.clone() {
            self.publish_message(message)
        }
    }

    pub fn get_client_id(&self) -> String {
        self.client_id.to_string()
    }

    pub fn equals(&self, client_id: String) -> bool {
        self.client_id == client_id
    }

    pub fn disconect(&mut self) {
        self.socket = None;
    }
}
