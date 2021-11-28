use crate::keepalive::keep_alive::KeepAlive;
use crate::keepalive::null_keep_alive::KeepAliveNull;
use crate::keepalive::trait_keep_alive::TraitKeepAlive;
use crate::paquetes::connect::Connect;
use crate::paquetes::default::Default;
use crate::paquetes::publish::Publish;
use crate::paquetes::subscribe::Subscribe;
use crate::stream::stream_handler::StreamType;
use crate::usermanager::user_manager_types::ChannelUserManager;
use std::sync::mpsc::Sender;

use super::publisher_suscriber::PublisherSuscriber;

pub struct PacketManager {
    client_id: String,
    keep_alive: Box<dyn TraitKeepAlive>,
    sender_stream: Sender<StreamType>,
    sender_user_manager: Sender<ChannelUserManager>,
    sender_to_disconect: Sender<(String, String)>,
    sender_topic_manager: Sender<PublisherSuscriber>,
}

impl PacketManager {
    pub fn init(
        sender_user_manager: Sender<ChannelUserManager>,
        sender_to_disconect: Sender<(String, String)>,
        sender_stream: Sender<StreamType>,
        sender_topic_manager: Sender<PublisherSuscriber>,
    ) -> Self {
        PacketManager {
            client_id: "".to_string(),
            sender_stream,
            sender_user_manager,
            keep_alive: KeepAliveNull::init(0, sender_to_disconect.clone()),
            sender_to_disconect,
            sender_topic_manager,
        }
    }

    pub fn get_control_packet_type(first_byte: u8) -> u8 {
        (0xF0 & first_byte) >> 4
    }

    pub fn set_client_id(&mut self, client_id: String) {
        self.client_id = client_id;
    }

    pub fn start_keep_alive(&mut self) {
        self.keep_alive
            .start_keep_alive(self.client_id.to_string(), "prueba".to_string());
    }

    pub fn set_keep_alive(&mut self, keep_alive: Box<dyn TraitKeepAlive>) {
        self.keep_alive = keep_alive;
    }

    pub fn process_connect_message(&mut self, bytes: &[u8]) {
        let connect = Connect::init(
            bytes,
            self.sender_stream.clone(),
            self.sender_user_manager.clone(),
        );
        self.set_client_id(connect.get_client_id());

        let keep_alive = match connect.get_keep_alive() {
            Some(some_keep_alive) => {
                KeepAlive::init(some_keep_alive, self.sender_to_disconect.clone())
            }
            None => KeepAliveNull::init(0, self.sender_to_disconect.clone()),
        };

        self.set_keep_alive(keep_alive);
        connect.send_response(self.sender_stream.clone());
    }

    pub fn process_publish_message(&mut self, bytes: &[u8]) {
        Publish::init(bytes)
            .send_message(&self.sender_topic_manager, self.client_id.to_owned())
            .send_response(self.sender_stream.clone());
    }

    pub fn process_subscribe_message(&mut self, bytes: &[u8]) {
        Subscribe::init(bytes)
            .subscribe_topic(
                self.sender_topic_manager.clone(),
                self.sender_user_manager.clone(),
                self.client_id.to_owned(),
            )
            .send_response(self.sender_stream.clone())
    }

    // TODO: validar que un paquete que no es connect, siempre tenga que estar ya conectado (haber hecho un connect packet previamente)
    pub fn process_message(&mut self, bytes: &[u8]) {
        let first_byte = bytes.get(0);
        match first_byte {
            Some(first_byte_ok) => match PacketManager::get_control_packet_type(*first_byte_ok) {
                1 => self.process_connect_message(bytes),
                3 => self.process_publish_message(bytes),
                8 => self.process_subscribe_message(bytes),
                _ => Default::init(bytes).send_response(self.sender_stream.clone()),
            },
            None => Default::init(bytes).send_response(self.sender_stream.clone()),
        };
    }
}
