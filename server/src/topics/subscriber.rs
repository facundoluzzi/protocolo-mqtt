
use std::{io::Write, net::TcpStream};

pub struct Subscriber<W> {
    socket: Option<W>,
    queue: Vec<String>,
    client_id: String,
}

impl<W> Clone for Subscriber<W> {
    fn clone(&self) -> Subscriber<&mut W> where W: Write {
        Subscriber {
            socket: if let Some(socket) = &self.socket {
                Some(&mut socket.clone())
            } else {
                None
            },
            queue: self.queue.clone(),
            client_id: self.client_id.clone(),
        }
    }
}

impl<W: ?Sized> Subscriber<W> {
    pub fn init(client_id: String, writer: &mut W) -> Subscriber<&mut W> where W: Write {
        // Subscriber {
        //     socket: Some(BufWriter::new(*socket)),
        //     queue: Vec::new(),
        //     client_id,
        // }
        Subscriber { queue: Vec::new(), client_id, socket: Some(writer) }
    }

    pub fn publish_message(&mut self, message: String) {
        // if let Some(socket) = &self.socket {
        //     socket.clone().write(&message.as_bytes());
        // } else {
        //     self.queue.push(message);
        // }
    }

    pub fn disconnect(&mut self) {
        // self.socket = None;
    }

    pub fn reconnect(&mut self, socket: TcpStream) {
        // self.socket = Some(socket);
        // for message in self.queue.clone() {
        //     self.publish_message(message)
        // }
    }

    pub fn equals(&self, client_id: String) -> bool {
        // self.client_id == client_id
        true
    }

    pub fn assign_socket(&self, stream: &std::net::TcpStream) {}

    pub fn delete_subscriber(&self, name: String) {}
}

#[cfg(test)]
mod tests {
    use std::io::BufWriter;
    use std::fs::File;
    use super::*;

    #[test]
    fn create_a_subscriber_and_receive_a_publish() {
        let file = File::create("prueba.txt").unwrap();
        let mut writer = BufWriter::new(file);
        let subscriber = Subscriber::init("1234".to_owned(), &mut writer);
        // subscriber.publish_message("message".to_owned());
        // const length_message: usize = "message".as_bytes().len();
        // let reading = stream.read(&mut [0; length_message]).unwrap();
        // assert_eq!(reading, "message".as_bytes());
    }

    // fn create_a_subscriber_and_disconnect_leave_the_socket_and_fill_queue() {
    //     let mut stream = TcpStream::connect("0.0.0.0:1883").unwrap();
    //     let subscriber = Subscriber::new(client_id, stream);
    //     subscriber.disconnect();
    //     subscriber.publish_message("message".to_owned());
    //     const length_message: usize = "message".as_bytes().len();
    //     let reading = stream.read(&mut [0; length_message]).unwrap();
    //     assert_eq!(subscriber.queue[0], "message".to_owned());
    // }

    // fn create_a_subscriber_and_disconnect_then_reconnect_and_receive_message_encolados() {
    //     let mut stream = TcpStream::connect("0.0.0.0:1883").unwrap();
    //     let subscriber = Subscriber::new("123".to_owned(), stream);
    //     subscriber.disconnect();
    //     subscriber.publish_message("message".to_owned());
    //     subscriber.
    //     let length_message = "message".as_bytes().len();
    //     let reading = stream.read(&mut [0; length_message]).unwrap();
    //     assert_eq!(reading, "message".as_bytes());
    // }
}
