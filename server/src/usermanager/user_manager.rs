use crate::enums::topic_manager::topic_message::TypeMessage;
use crate::enums::topic_manager::unsubscriberall::UnsubscriberAll;
use crate::enums::user_manager::user_manager_action::UserManagerAction;
use crate::packets::publish::Publish;
use crate::stream::stream_handler::StreamType;
use crate::topic::publisher_writer::ChannelPublisherWriter;
use crate::topic::publisher_writer::PublisherSubscriberAction::DisconnectPublisherSubscriber;
use crate::topic::publisher_writer::PublisherSubscriberAction::PublishMessagePublisherSubscriber;
use crate::topic::publisher_writer::PublisherSubscriberAction::ReconnectPublisherSubscriber;
use crate::topic::publisher_writer::PublisherWriter;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::{collections::HashMap, sync::mpsc::Sender};

pub struct UserManager {
    users: HashMap<String, (Sender<ChannelPublisherWriter>, bool, Option<Publish>)>,
    sender_topic_manager: Sender<TypeMessage>,
}

impl UserManager {
    pub fn init(sender_topic_manager: Sender<TypeMessage>) -> Sender<UserManagerAction> {
        let (sender_user_manager, receiver_user_manager): (
            Sender<UserManagerAction>,
            Receiver<UserManagerAction>,
        ) = mpsc::channel();

        let mut user_manager = UserManager {
            users: HashMap::new(),
            sender_topic_manager,
        };

        thread::spawn(move || {
            for receive in receiver_user_manager {
                match receive {
                    UserManagerAction::AddUserManager(user) => {
                        let client_id = user.get_client_id();
                        let sender_stream = user.get_sender_stream();
                        let clean_session = user.get_clean_session();

                        if let Some(usuario) = user_manager.find_user(client_id.to_string()) {
                            usuario
                                .send((ReconnectPublisherSubscriber, None, Some(sender_stream)))
                                .unwrap();
                        } else {
                            let publish: Option<Publish>;
                            if let Some(message) = user.get_will_message() {
                                publish = Some(user_manager.generate_will_publish(
                                    user.get_will_topic(),
                                    message,
                                    user.get_will_qos(),
                                    user.get_will_retain_message(),
                                ));
                            } else {
                                publish = None;
                            }
                            user_manager.add(
                                client_id.to_string(),
                                sender_stream.clone(),
                                clean_session,
                                publish,
                            );
                        };
                    }
                    UserManagerAction::DisconnectUserManager(user) => {
                        let client_id = user.get_client_id();
                        let disconnection_ungracefully = user.get_disconnection_type();
                        user_manager.disconnect(client_id, disconnection_ungracefully);
                    }
                    UserManagerAction::PublishMessageUserManager(user) => {
                        let client_id = user.get_client_id();
                        let message = user.get_message();
                        if let Some(sender_for_publish) = user_manager.get_sender(client_id) {
                            sender_for_publish
                                .send((PublishMessagePublisherSubscriber, Some(message), None))
                                .unwrap();
                        }
                    }
                }
            }
        });

        sender_user_manager
    }

    fn add(
        &mut self,
        client_id: String,
        stream: Sender<StreamType>,
        clean_session: bool,
        publish_packet: Option<Publish>,
    ) {
        let publisher_writer = PublisherWriter::init(stream);
        self.users
            .insert(client_id, (publisher_writer, clean_session, publish_packet));
    }

    fn generate_will_publish(
        &mut self,
        topic: String,
        message: String,
        qos: u8,
        retained_message: bool,
    ) -> Publish {
        let mut publish_bytes = 0b00110000 | (qos << 1);
        if retained_message {
            publish_bytes += 1;
        }
        let topic_bytes = topic.as_bytes();
        let payload = message.as_bytes();
        let remaining_length = (6 + payload.len() + topic_bytes.len()) as u8;
        let mut publish: Vec<u8> = vec![
            publish_bytes,
            remaining_length,
            0x00,
            topic_bytes.len() as u8,
        ];
        for topic in topic_bytes {
            publish.push(*topic);
        }
        publish.push(0x00); // Packet Identifier
        publish.push(0x0A);
        publish.push(0x00);
        publish.push(payload.len() as u8);
        for message in payload {
            publish.push(*message);
        }
        Publish::init(&publish).unwrap()
    }

    fn find_user(&self, client_id: String) -> Option<Sender<ChannelPublisherWriter>> {
        self.users.get(&client_id).map(|user| user.0.clone())
    }

    fn get_sender(&self, client_id: String) -> Option<Sender<ChannelPublisherWriter>> {
        if let Some(publisher_writer) = self.find_user(client_id) {
            Some(publisher_writer)
        } else {
            println!("Unexpected error: user not found in user manager");
            None
        }
    }

    fn disconnect(&mut self, client_id: String, disconnect_ungracefully: bool) {
        let (clean_session, channel_publisher_writer): (
            Option<bool>,
            Option<Sender<ChannelPublisherWriter>>,
        ) = match self.users.get(&client_id) {
            Some(user) => (Some(user.1), Some(user.0.clone())),
            None => (None, None),
        };
        if disconnect_ungracefully {
            self.publish_last_will_message(client_id.to_owned());
            let mut pb: Option<Sender<ChannelPublisherWriter>> = None;
            let mut cs: bool = false;
            if let Some(user) = self.users.get(&client_id) {
                pb = Some(user.0.clone());
                cs = user.1;
            }
            self.users
                .insert(client_id.to_owned(), (pb.unwrap(), cs, None));
        }

        if let Some(clean_session) = clean_session {
            if clean_session {
                if self.users.remove(&client_id).is_none() {
                    println!("Unexpected error");
                }
                let unsubscriber_all = UnsubscriberAll::init(client_id);
                self.sender_topic_manager
                    .send(TypeMessage::UnsubscriberAll(unsubscriber_all))
                    .unwrap();
            } else if let Some(channel) = channel_publisher_writer {
                let publisher_writer_cloned = channel;
                publisher_writer_cloned
                    .send((DisconnectPublisherSubscriber, None, None))
                    .unwrap();
            }
        }
    }

    fn publish_last_will_message(&mut self, client_id: String) {
        if let Some(user) = self.users.get(&client_id) {
            if let Some(publish) = &user.2 {
                publish
                    .send_message(self.sender_topic_manager.clone(), client_id.to_owned())
                    .unwrap();
            }
        }
    }
}
