use std::{hash::Hasher, io::Write, net::TcpStream, os::windows::thread, sync::mpsc::{self, Receiver, Sender}};

pub struct PublisherWriter {
    sender: Sender<String>,
    socket: TcpStream,
    client_id: String,
}

impl Clone for PublisherWriter {
    fn clone(&self) -> Self {
        PublisherWriter {
            sender: self.sender.clone(),
            socket: self.socket.try_clone().unwrap(),
            client_id: self.client_id,
        } 
    }
}

impl PublisherWriter {

    pub fn init(socket: TcpStream, client_id: String) -> PublisherWriter {
        let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();
        // crear un receiver
        let mut publisher = PublisherWriter {
            sender,
            socket,
            client_id,
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
        self.socket.write(&receive.as_bytes()).unwrap();
    }

    pub fn reconnect(&mut self, stream: TcpStream) {
        self.socket = stream;
        //     for message in self.queue.clone() {
        //         self.publish_message(message)
        //     }
    }

    pub fn equals(&self, client_id: String) -> bool {
        self.client_id == client_id
    }
}