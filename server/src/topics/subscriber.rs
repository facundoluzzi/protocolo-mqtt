use std::io::BufWriter;
use std::{io::Write, net::TcpStream};

#[derive(Debug)]
pub struct Subscriber {
    socket: Option<BufWriter<TcpStream>>,
    queue: Vec<String>,
    client_id: String,
}

impl Clone for Subscriber {
    fn clone(&self) -> Self {
        Subscriber {
            socket: if let Some(socket) = &self.socket {
                Some(*socket.clone())
            } else {
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
            socket: Some(BufWriter::new(socket)),
            queue: Vec::new(),
            client_id,
        }
    }

    pub fn publish_message(&mut self, message: String) {
        if let Some(socket) = &self.socket {
            socket.clone().write(&message.as_bytes());
        } else {
            self.queue.push(message);
        }
    }

    // pub fn disconnect(&mut self) {
    //     self.socket = None;
    // }

    // pub fn reconnect(&mut self, socket: TcpStream) {
    //     self.socket = Some(socket);
    //     for message in self.queue.clone() {
    //         self.publish_message(message)
    //     }
    // }

    // pub fn equals(&self, client_id: String) -> bool {
    //     self.client_id == client_id
    // }

    // pub fn assign_socket(&self, stream: &std::net::TcpStream) {}

    // pub fn delete_subscriber(&self, name: String) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn create_a_subscriber_and_receive_a_publish() {
        let stream = match TcpStream::connect("0.0.0.0:1883") {
            Ok(ok_stream) => ok_stream,
            Err(err) => panic!(err),
        };
        let subscriber = Subscriber::new("1234".to_owned(), stream);
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
