// use std::{hash::Hasher, io::Write, net::TcpStream, sync::mpsc::{self, Receiver, Sender}};

// pub struct PublisherWriter<W> {
//     sender: Sender<String>,
//     socket: Option<W>,
//     client_id: String,
//     queue: Vec<String>,
// }

// impl<W> Clone for PublisherWriter<W> {
//     fn clone(&self) -> PublisherWriter<&mut W> where W: Write {
//         PublisherWriter {
//             sender: self.sender.clone(),
//             socket: if let Some(socket) = &self.socket {
//                 Some(&mut socket.clone())
//             } else {
//                 None
//             },
//             client_id: self.client_id.to_string(),
//             queue: self.queue.clone(),
//         }
//     }
// }

// impl<W: Write + Sized + Send> PublisherWriter<W> {

//     pub fn init(socket: &'static mut W, client_id: String) -> PublisherWriter<W> where W: Write {
//         let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();
//         // crear un receiver
//         let mut publisher = PublisherWriter {
//             sender,
//             socket: Some(socket),
//             client_id,
//             queue: Vec::new(),
//         };
//         let publisher_cloned = publisher.clone();
//         std::thread::spawn(move || {
//             for receive in receiver {
//                 publisher.publish_message(receive);
//             }
//         });
//         publisher_cloned
//     }

//     pub fn get_sender(&self) -> Sender<String> {
//         self.sender.clone()
//     }

//     pub fn publish_message(&mut self, receive: String) {
//         if let Some(socket) = &self.socket {
//             socket.clone().write(&receive.as_bytes());
//         } else {
//             self.queue.push(receive);
//         }
//     }

//     pub fn reconnect(&mut self, stream: W) {
//         self.socket = Some(stream);
//             for message in self.queue.clone() {
//             self.publish_message(message)
//         }
//     }

//     pub fn equals(&self, client_id: String) -> bool {
//         self.client_id == client_id
//     }
// }
