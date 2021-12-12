use crate::enums::publish_autosend::add_autosend::AddAutoSend;
use crate::enums::publish_autosend::autosend_action::AutoSendAction;
use crate::enums::publish_autosend::remove_autosend::RemoveAutoSend;
use crate::helper::remaining_length::save_remaining_length;
use crate::stream::stream_handler::StreamType;
use crate::variable_header::publish_variable_header::get_variable_header;
use crate::{
    helper::publish_autosend::PublishAutoSend, stream::stream_handler::StreamAction::WriteStream,
};
use std::convert::TryInto;
use std::sync::mpsc::{self, Receiver, Sender};

pub enum PublisherSubscriberAction {
    PublishMessagePublisherSubscriber,
    ReconnectPublisherSubscriber,
    DisconnectPublisherSubscriber,
    StopPublishPublisherSubscriber,
}

pub type ChannelPublisherWriter = (
    PublisherSubscriberAction,
    Option<Vec<u8>>,
    Option<Sender<StreamType>>,
);

pub struct PublisherWriter {
    socket: Option<Sender<StreamType>>,
    queue: Vec<Vec<u8>>,
    publish_autosend: Sender<AutoSendAction>,
}

impl PublisherWriter {
    fn disconect(&mut self) {
        self.socket = None;
    }

    fn get_packet_identifier(&self, bytes: &[u8]) -> Vec<u8> {
        let bytes_rem_len = &bytes[1..bytes.len()];
        let (readed_index, _) = save_remaining_length(bytes_rem_len).unwrap();

        let init_variable_header = 1 + readed_index;

        let (_, packet_identifier, _) =
            get_variable_header(&bytes[init_variable_header..bytes.len()]).unwrap();
        packet_identifier[0..2]
            .try_into()
            .expect("slice with incorrect length")
    }

    pub fn init(socket: Sender<StreamType>) -> Sender<ChannelPublisherWriter> {
        let (sender, receiver): (
            Sender<ChannelPublisherWriter>,
            Receiver<ChannelPublisherWriter>,
        ) = mpsc::channel();

        let mut publisher = PublisherWriter {
            socket: Some(socket),
            queue: Vec::new(),
            publish_autosend: PublishAutoSend::init(sender.clone()),
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
                    PublisherSubscriberAction::DisconnectPublisherSubscriber => {
                        publisher.disconect();
                    }
                    PublisherSubscriberAction::StopPublishPublisherSubscriber => {
                        publisher.remove(receive.1.unwrap());
                    }
                };
            }
        });

        sender
    }

    fn publish_message(&mut self, receive: Vec<u8>) {
        if let Some(socket) = &self.socket {
            match socket.send((WriteStream, Some(receive.clone()), None, None)) {
                Ok(_) => {
                    self.publish_autosend
                        .send(AutoSendAction::Add(AddAutoSend::init(
                            self.get_packet_identifier(&receive),
                            receive,
                        )));
                }
                Err(_err) => {}
            }
        } else {
            self.queue.push(receive);
        }
    }

    fn reconnect(&mut self, stream: Sender<StreamType>) {
        self.socket = Some(stream);
        for message in self.queue.clone() {
            self.publish_message(message);
        }
    }

    fn remove(&mut self, packet_identifier: Vec<u8>) {
        self.publish_autosend
            .send(AutoSendAction::Remove(RemoveAutoSend::init(
                packet_identifier,
            )));
    }
}
