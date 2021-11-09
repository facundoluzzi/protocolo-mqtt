use std::{io::Write, net::{TcpListener, TcpStream}};

#[derive(Debug)]
pub struct Subscriber {
    socket: Option<TcpStream>,
    queue: Vec<String>
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
        }
    }
}


impl Subscriber {
    pub fn new(socket : TcpStream) -> Subscriber {
        Subscriber {
            socket: Some(socket),
            queue: Vec::new()
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
}
