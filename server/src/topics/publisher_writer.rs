use crate::stream::stream_handler::StreamAction::WriteStream;
use crate::stream::stream_handler::StreamType;
use std::sync::mpsc::{self, Receiver, Sender};

pub enum PublisherSubscriberAction {
    PublishMessagePublisherSubscriber,
    ReconnectPublisherSubscriber,
    DisconectPublisherSubscriber,
}

pub type ChannelPublisherWriter = (
    PublisherSubscriberAction,
    Option<String>,
    Option<Sender<StreamType>>,
);

pub struct PublisherWriter {
    socket: Option<Sender<StreamType>>,
    queue: Vec<String>,
}

impl PublisherWriter {
    pub fn init(socket: Sender<StreamType>) -> Sender<ChannelPublisherWriter> {
        let (sender, receiver): (
            Sender<ChannelPublisherWriter>,
            Receiver<ChannelPublisherWriter>,
        ) = mpsc::channel();

        let mut publisher = PublisherWriter {
            socket: Some(socket),
            queue: Vec::new(),
        };

        std::thread::spawn(move || {
            for receive in receiver {
                let action = receive.0;
                match action {
                    PublisherSubscriberAction::PublishMessagePublisherSubscriber => {
                        publisher.publish_message(receive.1.unwrap());
                    }
                    PublisherSubscriberAction::ReconnectPublisherSubscriber => {
                        publisher.reconnect(receive.2.unwrap());
                    }
                    PublisherSubscriberAction::DisconectPublisherSubscriber => {
                        publisher.disconect();
                    }
                };
            }
        });
        sender.clone()
    }

    // pub fn get_sender(&self) -> Sender<ChannelPublisherWriter> {
    //     self.sender.clone()
    // }

    fn publish_message(&mut self, receive: String) {
        if let Some(socket) = &self.socket {
            let new_vec: Vec<u8> = receive.as_bytes().to_vec();
            match socket.send((WriteStream, Some(new_vec), None)) {
                Ok(_) => {}
                Err(_err) => {}
            }
        } else {
            self.queue.push(receive);
        }
    }

    fn reconnect(&mut self, stream: Sender<StreamType>) {
        self.socket = Some(stream);
        for message in self.queue.clone() {
            self.publish_message(message)
        }
    }

    fn disconect(&mut self) {
        self.socket = None;
    }
}
