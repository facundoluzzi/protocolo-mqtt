use std::{hash::Hasher, io::Write, net::TcpStream, sync::mpsc::{self, Receiver, Sender}};
use crate::writers::publisher_writer::PublisherWriter;


pub struct TcpWriter {
    sender: Sender<String>,
    socket: Option<TcpStream>,
    client_id: String,
    queue: Vec<String>,
}

impl Clone for TcpWriter{
    fn clone(&self) -> TcpWriter{
        TcpWriter {
            sender: self.sender.clone(),
            socket: if let Some(socket) = &self.socket {
                Some(socket.try_clone().unwrap())
            } else {
                None
            },
            client_id: self.client_id.to_string(),
            queue: self.queue.clone(),
        }
    }
}

impl TcpWriter {
    pub fn init(socket:TcpStream, client_id: String) -> TcpWriter {
        let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();
        // crear un receiver
        let mut publisher = TcpWriter {
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
    
    pub fn reconnect(&mut self, stream: TcpStream) {
        self.socket = Some(stream);
            for message in self.queue.clone() {
            self.publish_message(message)
        }
    }
}

impl PublisherWriter for TcpWriter {
    fn get_sender(&self) -> Sender<String> {
        self.sender.clone()
    }

    fn publish_message(&mut self, receive: String) {
        if let Some(socket) = &self.socket {
            socket.clone().write(&receive.as_bytes());
        } else {
            self.queue.push(receive);
        }
    }


    fn equals(&self, client_id: String) -> bool {
        self.client_id == client_id
    }
}
