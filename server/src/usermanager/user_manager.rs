use crate::stream::stream_handler::StreamType;
use crate::topics::publisher_writer::ChannelPublisherWriter;
use crate::topics::publisher_writer::PublisherSubscriberAction::DisconectPublisherSubscriber;
use crate::topics::publisher_writer::PublisherSubscriberAction::PublishMessagePublisherSubscriber;
use crate::topics::publisher_writer::PublisherSubscriberAction::ReconnectPublisherSubscriber;
use crate::topics::publisher_writer::PublisherWriter;
use crate::topics::topic_types::TypeTopicManager;
use crate::topics::unsubscriberall::UnsubscriberAll;
use crate::usermanager::user_manager_action::UserManagerAction;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::{collections::HashMap, sync::mpsc::Sender};

pub struct UserManager {
    users: HashMap<String, (Sender<ChannelPublisherWriter>, bool)>,
    sender_topic_manager: Sender<TypeTopicManager>,
}

impl UserManager {
    pub fn init(sender_topic_manager: Sender<TypeTopicManager>) -> Sender<UserManagerAction> {
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

                let unsubscriber_all = UnsubscriberAll::init(client_id);

                self.sender_topic_manager
                    .send(TypeTopicManager::UnsubscriberAll(unsubscriber_all))
                    .unwrap();
            } else if let Some(channel) = channel_publisher_writer {
                let publisher_writer_cloned = channel;
                publisher_writer_cloned
                    .send((DisconectPublisherSubscriber, None, None))
                    .unwrap();
            }
        }
    }
}
