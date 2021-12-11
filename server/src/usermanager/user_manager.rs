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
                            user_manager.add(
                                client_id.to_string(),
                                sender_stream.clone(),
                                clean_session,
                            );
                        };
                    }
                    UserManagerAction::DisconnectUserManager(user) => {
                        let client_id = user.get_client_id();
                        user_manager.disconnect(client_id);
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

    fn add(&mut self, client_id: String, stream: Sender<StreamType>, clean_session: bool) {
        let publisher_writer = PublisherWriter::init(stream);
        let packet = self.generate_will_publish(topic, message, qos, retained_message);
        self.users
            .insert(client_id, (publisher_writer, clean_session));
    }

    fn generate_will_publish(&mut self,topic: String, message: String, qos: u8, retained_message: bool){
        let publish_bytes = [
            0x32, // tiene la información del packet type 0011, dup flag + qos flag + retain flag
            0x0C, // remaining length
            0x00, 0x03, 0x61, 0x2F, 0x62, // topic name
            0x00, 0x0A, // packet identifier
            0x00, 0x03, 0x61, 0x2F, 0x61, // payload
        ];
        let publish_bytes = 0b00110000 | (qos << 1);
        if retained_message {
            publish_bytes = publish_bytes + 1;
        }
        let topic_bytes = topic.as_bytes();
        let packet_identifier = vec![0x00, 0x0A];
        let payload = message.as_bytes();
        let remaining_length = (2 + packet_identifier.len() + payload.len()) as u8;
        let mut publish: Vec<u8> = vec![publish_bytes, remaining_length, 0x00, topic_bytes.len() as u8];
        for i in topic_bytes.len() {
            publish.push(topic_bytes[i]);
        }
        publish.append(topic_bytes.to_vec()); 
        publish.append(packet_identifier.to_vec());
        publish.append(payload.to_vec());
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

    fn disconnect(&mut self, client_id: String) {
        let (clean_session, channel_publisher_writer): (
            Option<bool>,
            Option<Sender<ChannelPublisherWriter>>,
        ) = match self.users.get(&client_id) {
            Some(user) => (Some(user.1), Some(user.0.clone())),
            None => (None, None),
        };

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
}
