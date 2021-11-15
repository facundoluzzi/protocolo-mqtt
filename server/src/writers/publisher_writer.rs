use std::{hash::Hasher, io::Write, net::TcpStream, sync::mpsc::{self, Receiver, Sender}};

pub trait PublisherWriter {
    fn publish_message(&mut self, receive: String);
    fn get_sender(&self) -> Sender<String>;
    fn equals(&self, client_id: String) -> bool;
}