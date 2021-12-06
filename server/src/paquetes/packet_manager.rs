use crate::logs::logger::Logger;
use crate::paquetes::connect::Connect;
use crate::paquetes::default::Default;
use crate::paquetes::disconnect::Disconnect;
use crate::paquetes::pingreq;
use crate::paquetes::publish::Publish;
use crate::paquetes::subscribe::Subscribe;
use crate::paquetes::unsubscribe::Unsubscribe;
use crate::stream::stream_handler::StreamType;
use crate::topics::topic_types::TypeTopicManager;
use crate::usermanager::user_manager_types::ChannelUserManager;
use std::sync::mpsc::Sender;

pub struct PacketManager {
    client_id: String,
    sender_stream: Sender<StreamType>,
    sender_user_manager: Sender<ChannelUserManager>,
    sender_topic_manager: Sender<TypeTopicManager>,
    logger: Logger,
}

impl PacketManager {
    pub fn init(
        sender_user_manager: Sender<ChannelUserManager>,
        sender_stream: Sender<StreamType>,
        sender_topic_manager: Sender<TypeTopicManager>,
        logger: Logger,
    ) -> Self {
        PacketManager {
            client_id: "".to_string(),
            sender_stream,
            sender_user_manager,
            sender_topic_manager,
            logger,
        }
    }

    pub fn get_control_packet_type(first_byte: u8) -> u8 {
        (0xF0 & first_byte) >> 4
    }

    pub fn set_client_id(&mut self, client_id: String) {
        self.client_id = client_id;
    }

    fn get_client_id(&self) -> String {
        self.client_id.to_string()
    }

    fn process_connect_message(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.logger.info("proccessing connect packet".to_string());

        let connect = Connect::init(
            bytes,
            self.sender_stream.clone(),
            self.sender_user_manager.clone(),
        );

        match connect {
            Ok(connect_result) => {
                self.set_client_id(connect_result.get_client_id());
                connect_result
                    .send_response(self.sender_stream.clone(), self.sender_user_manager.clone())?;
                Ok(())
            }
            Err(err_msg) => {
                self.logger.info(format!(
                    "Unexpected error processing connect packet: {}",
                    err_msg
                ));
                Disconnect::disconnect_user(
                    self.client_id.to_owned(),
                    self.sender_user_manager.clone(),
                    self.sender_stream.clone(),
                );
                Ok(())
            }
        }
    }

    pub fn process_disconnect_message(&mut self) {
        Disconnect::disconnect_user(
            self.client_id.to_owned(),
            self.sender_user_manager.clone(),
            self.sender_stream.clone(),
        );
    }

    pub fn process_publish_message(&mut self, bytes: &[u8]) {
        self.logger.info("proccessing publish packet".to_string());

        Publish::init(bytes)
            .send_message(&self.sender_topic_manager, self.get_client_id())
            .send_response(self.sender_stream.clone());
    }

    fn process_subscribe_message(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.logger.info("proccessing subscribe packet".to_string());
        let subscribe = Subscribe::init(bytes);
        match subscribe {
            Ok(mut created_subscribe) => {
                let subscribe_topic_response = created_subscribe.subscribe_topic(
                    self.sender_topic_manager.clone(),
                    self.sender_user_manager.clone(),
                    self.get_client_id(),
                );

                match subscribe_topic_response {
                    Ok(subscribed_topic) => {
                        subscribed_topic.send_response(self.sender_stream.clone());
                        Ok(())
                    }
                    Err(_) => Err("".to_string()),
                }
            }
            Err(err) => {
                let message = format!("Unexpected error processing connect packet: {}", err);
                self.logger.info(message.to_string());
                Disconnect::disconnect_user(
                    self.client_id.to_owned(),
                    self.sender_user_manager.clone(),
                    self.sender_stream.clone(),
                );
                Ok(())
            }
        }
    }

    fn process_unsubscribe_message(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.logger.info("proccessing subscribe packet".to_string());

        let unsubscribe = Unsubscribe::init(bytes);
        match unsubscribe {
            Ok(mut created_unsubscribe) => {
                let unsubscribe_topic_response = created_unsubscribe
                    .unsubscribe_topic(self.sender_topic_manager.clone(), self.get_client_id());

                match unsubscribe_topic_response {
                    Ok(subscribed_topic) => {
                        subscribed_topic.send_response(self.sender_stream.clone());
                        Ok(())
                    }
                    Err(_) => Err("".to_string()),
                }
            }
            Err(err) => {
                let message = format!("Unexpected error processing connect packet: {}", err);
                self.logger.info(message.to_string());
                Disconnect::disconnect_user(
                    self.client_id.to_owned(),
                    self.sender_user_manager.clone(),
                    self.sender_stream.clone(),
                );
                Ok(())
            }
        }
    }

    fn process_pingreq_message(&self) {
        pingreq::send_response(self.sender_stream.clone());
    }

    pub fn process_message(&mut self, bytes: &[u8]) -> Result<(), String> {
        let first_byte = bytes.get(0);
        match first_byte {
            Some(first_byte_ok) => {
                let packet_type = PacketManager::get_control_packet_type(*first_byte_ok);

                self.logger.info(format!("Packet type: {}", packet_type));

                match packet_type {
                    1 => self.process_connect_message(bytes)?,
                    3 => self.process_publish_message(bytes),
                    8 => self.process_subscribe_message(bytes)?,
                    10 => self.process_unsubscribe_message(bytes)?,
                    12 => self.process_pingreq_message(),
                    14 => self.process_disconnect_message(),
                    _ => Default::init(bytes).send_response(self.sender_stream.clone()),
                }
            }
            None => Default::init(bytes).send_response(self.sender_stream.clone()),
        };
        Ok(())
    }
}
