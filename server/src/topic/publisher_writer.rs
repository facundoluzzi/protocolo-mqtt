use crate::enums::publish_autosend::add_autosend::AddAutoSend;
use crate::enums::publish_autosend::autosend_action::AutoSendAction;
use crate::enums::publish_autosend::remove_autosend::RemoveAutoSend;
use crate::enums::publisher_writter::channel::ChannelPublisherWriter;
use crate::enums::publisher_writter::reconnect_stream::ReconnectStream;
use crate::enums::publisher_writter::stop_publish_to_stream::StopPublishToStream;
use crate::helper::remaining_length::save_remaining_length;
use crate::stream::stream_handler::StreamType;
use crate::variable_header::publish_variable_header::get_variable_header;
use crate::{
    helper::publish_autosend::PublishAutoSend, stream::stream_handler::StreamAction::WriteStream,
};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

pub enum PublisherSubscriberAction {
    PublishMessagePublisherSubscriber,
    ReconnectPublisherSubscriber,
    DisconnectPublisherSubscriber,
    StopPublishPublisherSubscriber,
}

pub struct PublisherWriter {
    socket: Option<Sender<StreamType>>,
    queue: Vec<Vec<u8>>,
    publish_autosend: Sender<AutoSendAction>,
}

impl PublisherWriter {
    /// Recibe un sender de Stream Handler y lanza un thread para quedarse escuchando por nuevos eventos.
    /// Estos eventos pueden ser publish, disconnect, reconnect y stop to publish
    pub fn init(socket: Sender<StreamType>) -> Sender<ChannelPublisherWriter> {
        let (sender, receiver): (
            Sender<ChannelPublisherWriter>,
            Receiver<ChannelPublisherWriter>,
        ) = mpsc::channel();

        let publisher = PublisherWriter {
            socket: Some(socket),
            queue: Vec::new(),
            publish_autosend: PublishAutoSend::init(sender.clone()),
        };
        publisher.throw_thread_to_listen_events(receiver);
        sender
    }

    fn throw_thread_to_listen_events(mut self, receiver: Receiver<ChannelPublisherWriter>) {
        thread::spawn(move || {
            for event in receiver {
                match event {
                    ChannelPublisherWriter::Publish(publish) => {
                        self.publish_message(publish.get_message())
                    }
                    ChannelPublisherWriter::Reconnect(reconnect) => self.reconnect(reconnect),
                    ChannelPublisherWriter::Disconnect => self.disconnect(),
                    ChannelPublisherWriter::StopToPublish(stop_publish) => {
                        self.remove(stop_publish)
                    }
                };
            }
        });
    }

    fn reconnect(&mut self, reconnect: ReconnectStream) {
        self.socket = Some(reconnect.get_sender());
        for message in self.queue.clone() {
            self.publish_message(message);
        }
    }

    fn disconnect(&mut self) {
        self.socket = None;
    }

    fn remove(&mut self, stop_publish: StopPublishToStream) {
        let packet_id = stop_publish.get_packet_id();
        let remove_autosend = RemoveAutoSend::init(packet_id);
        let action = AutoSendAction::Remove(remove_autosend);
        let result = self.publish_autosend.send(action);
        if let Err(err) = result {
            println!("Unexpected error stopping to publish: {}", err);
        }
    }

    /**
     * esto se usa solamente en qos 1, por lo cual si no tiene packet id, lanza panic.
     */
    fn get_packet_identifier(&self, bytes: &[u8]) -> Result<Vec<u8>, String> {
        println!("LLEGO ACA");
        let (readed_index, _): (usize, usize) = save_remaining_length(&bytes[1..bytes.len()])?;
        let init_variable_header = 1 + readed_index;
        let variable_header = &bytes[init_variable_header..bytes.len()];
        let (_, packet_id, _) = get_variable_header(variable_header, 1)?;
        if let Some(packet) = packet_id {
            Ok(packet.to_vec())
        } else {
            panic!("Unexpected error: unnecessary packet identifier in qos 1");
        }
    }

    fn publish_message(&mut self, message: Vec<u8>) {
        if let Some(socket) = &self.socket {
            match socket.send((WriteStream, Some(message.clone()), None, None)) {
                Ok(_) => {
                    self.publish(message);
                }
                Err(_err) => {}
            }
        } else {
            self.queue.push(message);
        }
    }

    fn publish(&self, message: Vec<u8>) {
        let qos_is_1 = 0b00000010 & message[0] > 0;
        if qos_is_1 {
            if let Ok(packet_id) = self.get_packet_identifier(&message) {
                let autosend = AddAutoSend::init(packet_id, message);
                let action = AutoSendAction::Add(autosend);
                let result = self.publish_autosend.send(action);
                if let Err(err) = result {
                    println!("Unexpected error sending autosend: {}", err);
                }
            }
        }
    }
}
