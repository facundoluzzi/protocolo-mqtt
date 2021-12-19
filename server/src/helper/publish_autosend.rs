use crate::config_parser::ServerConfigs;
use crate::enums::publisher_writter::channel::ChannelPublisherWriter;
use crate::enums::publisher_writter::publish_to_stream::PublishToStream;
use core::time;
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::spawn;

use crate::enums::publish_autosend::autosend_action::AutoSendAction;
use crate::enums::publish_autosend::publish_all_autosend::PublishAllAutoSend;

type SenderPublishAutoSend = (Sender<AutoSendAction>, Receiver<AutoSendAction>);

/// Contiene publish_packets guardados
pub struct PublishAutoSend {
    publish_packets: HashMap<Vec<u8>, Vec<u8>>,
}

impl PublishAutoSend {
    fn throw_thread_to_publish_all(dup_time: u64, sender: Sender<AutoSendAction>) {
        spawn(move || loop {
            std::thread::sleep(time::Duration::from_secs(dup_time));
            let result = sender.send(AutoSendAction::PublishAll(PublishAllAutoSend::init()));
            if let Err(error) = result {
                println!("{:?}", error);
            };
        });
    }

    fn throw_thread_to_listen_events(
        mut self,
        receiver: Receiver<AutoSendAction>,
        sender_publisher_writer: Sender<ChannelPublisherWriter>,
    ) {
        spawn(move || {
            for type_action in receiver {
                match type_action {
                    AutoSendAction::Add(operation) => {
                        self.add(operation.get_packet_identifier(), operation.get_publish())
                    }
                    AutoSendAction::Remove(operation) => {
                        self.remove(operation.get_packet_identifier())
                    }
                    AutoSendAction::PublishAll(_) => {
                        self.publish_all(sender_publisher_writer.clone())
                    }
                }
            }
        });
    }

    /// Recibe un sender de publisher writer. Reprocesa los publish cada 5 segundos
    /// lanza un thread que se queda escuchando por eventos y otro thread
    /// para poder cortar el envío constante de paquetes.
    pub fn init(sender_publisher_writer: Sender<ChannelPublisherWriter>) -> Sender<AutoSendAction> {
        let config = ServerConfigs::obtain_configurations("./server.conf".to_string());
        let dup_time = config
            .get_conf_named("dup_time".to_string())
            .parse::<u64>()
            .unwrap_or(5_u64);
        let (sender, receiver): SenderPublishAutoSend = mpsc::channel();
        let publish_autosend = PublishAutoSend {
            publish_packets: HashMap::new(),
        };
        publish_autosend.throw_thread_to_listen_events(receiver, sender_publisher_writer);
        PublishAutoSend::throw_thread_to_publish_all(dup_time, sender.clone());
        sender
    }

    /// Agrega un paquete publish para enviar cada 5 segundos, si tiene qos 1
    pub fn add(&mut self, packet_identifier: Vec<u8>, receive: Vec<u8>) {
        let byte = receive[0];
        let publish_qos_1 = byte & 0b00000010 > 0;
        if publish_qos_1 {
            let first_byte = byte | 0b00001000;
            let mut packet = vec![first_byte];
            packet.append(&mut receive[1..receive.len()].to_vec());
            self.publish_packets.insert(packet_identifier, packet);
        }
    }

    /// Remueve un paquete con el packet id, para que el paquete deje de ser enviado.
    /// Esto sucede cuando el broker recibe un puback
    pub fn remove(&mut self, packet_identifier: Vec<u8>) {
        self.publish_packets.remove(&packet_identifier);
    }

    /// publica a todos los mensajes guardados
    pub fn publish_all(&mut self, sender: Sender<ChannelPublisherWriter>) {
        for publish in self.publish_packets.clone() {
            let publish_to_stream = PublishToStream::init(publish.1);
            let result = sender.send(ChannelPublisherWriter::Publish(publish_to_stream));
            if let Err(msg) = result {
                println!("Unexpected error: {}", msg);
            };
        }
    }
}
