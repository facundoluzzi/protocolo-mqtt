use std::{
    io::Write,
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
};

pub struct PublisherWriter {
    sender: Sender<String>,
    socket: Option<TcpStream>,
    client_id: String,
    queue: Vec<String>,
}

impl Clone for PublisherWriter {
    fn clone(&self) -> PublisherWriter {
        PublisherWriter {
            sender: self.sender.clone(),
            socket: Some(self.socket.as_ref().unwrap().try_clone().expect("Error")),
            client_id: self.client_id.to_string(),
            queue: self.queue.clone(),
        }
    }
}

impl PublisherWriter {
    pub fn init(socket: TcpStream, client_id: String) -> PublisherWriter {
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
            if let Ok(a) = socket.clone().write(&receive.as_bytes()) {
                println!("Enviado")
            } else {
                println!("Error")
            };
        } else {
            self.queue.push(receive);
        }
    }

    pub fn reconnect(&mut self, stream: TcpStream) {
        self.socket = Some(stream);
        for message in self.queue.clone() {
            self.publish_message(message)
        }
    }

    pub fn equals(&self, client_id: String) -> bool {
        self.client_id == client_id
    }
}
