use std::{io::Write, net::{TcpListener, TcpStream}};

#[derive(Debug)]
pub struct Subscriber {
    socket: Option<TcpStream>,
    queue: Vec<String>,
    client_id: String,
}

impl Clone for Subscriber {
    fn clone(&self) -> Self {
        Subscriber {
            socket: if let Some(socket) = &self.socket{
                Some(socket.try_clone().unwrap())
            }else{
                None
            },
            queue: self.queue.clone(),
            client_id: self.client_id.clone(),
        }
    }
}

impl Subscriber {
    pub fn new(client_id: String, socket: TcpStream) -> Subscriber {
        Subscriber {
            socket: Some(socket),
            queue: Vec::new(),
            client_id,
        }
    }

    pub fn publish_message(&mut self, message: String) {
        if let Some(socket) = &mut self.socket {
            socket.write(&message.as_bytes());
        } else {
            self.queue.push(message);
        }
    }

    pub fn disconnect(&self){
        self.socket = None;

    }

    pub fn reconnect(&mut self, socket: TcpStream){
        self.socket = Some(socket);
        for message in self.queue {
            self.publish_message(message)
        }
    }

    pub fn equals(&self, client_id: String) -> bool {
        self.client_id == client_id
    }
}
