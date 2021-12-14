use crate::enums::topic_manager::topic_message::TypeMessage;

use crate::enums::user_manager::user_manager_action::UserManagerAction;
use crate::logs::logger::Logger;
use crate::packets::connect::Connect;
use crate::packets::default::Default;
use crate::packets::disconnect::Disconnect;
use crate::packets::pingreq;
use crate::packets::publish::Publish;
use crate::packets::subscribe::Subscribe;
use crate::packets::unsubscribe::Unsubscribe;
use crate::stream::stream_handler::StreamType;
use std::sync::mpsc::Sender;

use super::puback::Puback;

pub struct PacketManager {
    client_id: String,
    sender_stream: Sender<StreamType>,
    sender_user_manager: Sender<UserManagerAction>,
    sender_topic_manager: Sender<TypeMessage>,
    logger: Logger,
    is_disconnected: bool,
}

impl PacketManager {
    pub fn init(
        sender_user_manager: Sender<UserManagerAction>,
        sender_stream: Sender<StreamType>,
        sender_topic_manager: Sender<TypeMessage>,
        logger: Logger,
    ) -> Self {
        PacketManager {
            client_id: "".to_string(),
            sender_stream,
            sender_user_manager,
            sender_topic_manager,
            logger,
            is_disconnected: true,
        }
    }

    pub fn get_control_packet_type(first_byte: u8) -> u8 {
        (0xF0 & first_byte) >> 4
    }

    pub fn set_client_id(&mut self, client_id: String) {
        self.client_id = client_id;
    }

    pub fn get_client_id(&self) -> String {
        self.client_id.to_string()
    }

    pub fn get_sender_stream(&self) -> Sender<StreamType> {
        self.sender_stream.clone()
    }

    pub fn get_sender_user_manager(&self) -> Sender<UserManagerAction> {
        self.sender_user_manager.clone()
    }

    pub fn get_sender_topic_manager(&self) -> Sender<TypeMessage> {
        self.sender_topic_manager.clone()
    }

    pub fn is_disconnected(&self) -> bool {
        self.is_disconnected
    }

    fn connect(&mut self) {
        self.is_disconnected = false;
    }

    fn disconnect(&mut self) {
        self.is_disconnected = true;
    }

    fn process_connect_message(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.logger.info("proccessing connect packet".to_string());

        if let Err(err) = Connect::process_message(bytes, self) {
            let message_to_log = "Unexpected error processing connect packet:";
            self.logger.info(format!("{}: {}", message_to_log, err));
            Disconnect::disconnect_user(
                self.client_id.to_owned(),
                self.sender_user_manager.clone(),
                self.sender_stream.clone(),
            );
            self.disconnect();
            Err(err)
        } else {
            self.connect();
            Ok(())
        }
    }

    pub fn process_publish_message(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.logger.info("proccessing publish packet".to_string());

        if let Err(err) = Publish::process_message(bytes, self) {
            let message_to_log = "Unexpected error processing publish packet:";
            self.logger.info(format!("{}: {}", message_to_log, err));
            Disconnect::disconnect_user(
                self.client_id.to_owned(),
                self.sender_user_manager.clone(),
                self.sender_stream.clone(),
            );
            self.disconnect();
            Err(err)
        } else {
            Ok(())
        }
    }

    pub fn process_disconnect_message(&mut self) -> Result<(), String> {
        Disconnect::disconnect_user(
            self.client_id.to_owned(),
            self.sender_user_manager.clone(),
            self.sender_stream.clone(),
        );
        self.disconnect();
        Ok(())
    }

    fn process_subscribe_message(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.logger.info("proccessing subscribe packet".to_string());

        if let Err(err) = Subscribe::process_message(bytes, self) {
            let message_to_log = "Unexpected error subscribe publish packet:";
            self.logger.info(format!("{}: {}", message_to_log, err));
            Disconnect::disconnect_ungracefully(
                self.client_id.to_owned(),
                self.sender_user_manager.clone(),
                self.sender_stream.clone(),
            );
            self.disconnect();
            Err(err)
        } else {
            Ok(())
        }
    }

    fn process_unsubscribe_message(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.logger
            .info("proccessing unsubscribe packet".to_string());
        if let Err(err) = Unsubscribe::process_message(bytes, self) {
            let message = format!("Unexpected error processing unsubscribe packet: {}", err);
            self.logger.info(message);
            Disconnect::disconnect_user(
                self.client_id.to_owned(),
                self.sender_user_manager.clone(),
                self.sender_stream.clone(),
            );
            Err(err)
        } else {
            Ok(())
        }
    }

    fn process_pingreq_message(&mut self) -> Result<(), String> {
        if let Err(err) = pingreq::send_response(self.sender_stream.clone()) {
            let message_to_log = "Unexpected error processing pingreq packet:";
            self.logger.info(format!("{}: {}", message_to_log, err));
            Disconnect::disconnect_user(
                self.client_id.to_owned(),
                self.sender_user_manager.clone(),
                self.sender_stream.clone(),
            );
            self.disconnect();
            Err(err)
        } else {
            Ok(())
        }
    }

    fn process_puback(&self, bytes: &[u8]) {
        Puback::init(bytes)
            .stop_publish(self.client_id.to_owned(), self.sender_user_manager.clone());
    }

    pub fn process_message(&mut self, bytes: &[u8]) -> Result<(), String> {
        let first_byte = bytes.get(0);
        match first_byte {
            Some(first_byte_ok) => {
                let packet_type = PacketManager::get_control_packet_type(*first_byte_ok);

                self.logger.info(format!("Packet type: {}", packet_type));
                match packet_type {
                    1 => self.process_connect_message(bytes)?,
                    3 => self.process_publish_message(bytes)?,
                    4 => self.process_puback(bytes),
                    8 => self.process_subscribe_message(bytes)?,
                    10 => self.process_unsubscribe_message(bytes)?,
                    12 => self.process_pingreq_message()?,
                    14 => self.process_disconnect_message()?,
                    _ => Default::init(bytes).send_response(self.sender_stream.clone()),
                }
            }
            None => Default::init(bytes).send_response(self.sender_stream.clone()),
        };
        Ok(())
    }
}
