use crate::helper::publisher_subscriber_code::PublisherSubscriberCode;
use crate::paquetes::publisher_suscriber::PublisherSuscriber;
use crate::stream::stream_handler::StreamType;
use crate::topics::publisher_writer::ChannelPublisherWriter;
use crate::topics::publisher_writer::PublisherSubscriberAction::DisconectPublisherSubscriber;
use crate::topics::publisher_writer::PublisherSubscriberAction::PublishMessagePublisherSubscriber;
use crate::topics::publisher_writer::PublisherSubscriberAction::ReconnectPublisherSubscriber;
use crate::topics::publisher_writer::PublisherWriter;
use crate::usermanager::user_manager_action::UserManagerAction::{
    AddUserManager, DisconnectUserManager, PublishMessageUserManager,
};
use crate::usermanager::user_manager_types::ChannelUserManager;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::{collections::HashMap, sync::mpsc::Sender};

pub struct UserManager {
    users: HashMap<String, (Sender<ChannelPublisherWriter>, bool)>,
    sender_topic_manager: Sender<PublisherSuscriber>,
}

impl UserManager {
    pub fn init(sender_topic_manager: Sender<PublisherSuscriber>) -> Sender<ChannelUserManager> {
        let (sender_user_manager, receiver_user_manager): (
            Sender<ChannelUserManager>,
            Receiver<ChannelUserManager>,
        ) = mpsc::channel();

        let mut user_manager = UserManager {
            users: HashMap::new(),
            sender_topic_manager,
        };

        thread::spawn(move || {
            for receive in receiver_user_manager {
                let action = receive.0;
                match action {
                    AddUserManager => {
                        let client_id = receive.1;
                        let sender_stream = receive.2.unwrap();
                        let clean_session = receive.3.unwrap();
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
                    DisconnectUserManager => {
                        let client_id = receive.1;
                        user_manager.disconnect(client_id);
                    }
                    PublishMessageUserManager => {
                        let client_id = receive.1;
                        let message = receive.4.unwrap();
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
        self.users
            .insert(client_id, (publisher_writer, clean_session));
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
                let publisher_subscriber = PublisherSuscriber::new(
                    "".to_owned(),
                    "".to_owned(),
                    PublisherSubscriberCode::UnsubscriberAll,
                    None,
                    client_id.to_owned(),
                    None,
                );
                self.sender_topic_manager
                    .send(publisher_subscriber)
                    .unwrap();
            } else {
                if let Some(channel) = channel_publisher_writer {
                    let publisher_writer_cloned = channel;
                    publisher_writer_cloned
                        .send((DisconectPublisherSubscriber, None, None))
                        .unwrap();
                }
            }
        }
    }
}
