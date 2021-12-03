use crate::stream::stream_handler::StreamType;
use crate::topics::publisher_writer::ChannelPublisherWriter;
use crate::topics::publisher_writer::PublisherSubscriberAction::DisconectPublisherSubscriber;
use crate::topics::publisher_writer::PublisherSubscriberAction::PublishMessagePublisherSubscriber;
use crate::topics::publisher_writer::PublisherSubscriberAction::ReconnectPublisherSubscriber;
use crate::topics::publisher_writer::PublisherWriter;
use crate::usermanager::user_manager_action::UserManagerAction::{
    AddUserManager, DeleteUserManager, DisconectUserManager, PublishMessageUserManager,
};
use crate::usermanager::user_manager_types::ChannelUserManager;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::{collections::HashMap, sync::mpsc::Sender};

pub struct UserManager {
    users: HashMap<String, (Sender<ChannelPublisherWriter>, bool)>,
}

impl UserManager {
    pub fn init() -> Sender<ChannelUserManager> {
        let (sender_user_manager, receiver_user_manager): (
            Sender<ChannelUserManager>,
            Receiver<ChannelUserManager>,
        ) = mpsc::channel();

        let mut user_manager = UserManager {
            users: HashMap::new(),
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
                    DeleteUserManager => {
                        let client_id = receive.1;
                        user_manager.delete_user(client_id);
                    }
                    DisconectUserManager => {
                        let client_id = receive.1;
                        user_manager.disconect(client_id);
                    }
                    PublishMessageUserManager => {
                        let client_id = receive.1;
                        let message = receive.4.unwrap();
                        let sender_for_publish = user_manager.get_sender(client_id).unwrap();
                        sender_for_publish
                            .send((PublishMessagePublisherSubscriber, Some(message), None))
                            .unwrap();
                    }
                }
            }
        });

        sender_user_manager
    }

    fn add(&mut self, client_id: String, stream: Sender<StreamType>, clean_session: bool) {
        let publisher_writer = PublisherWriter::init(stream.clone());
        self.users
            .insert(client_id, (publisher_writer, clean_session));
    }

    fn find_user(&self, client_id: String) -> Option<Sender<ChannelPublisherWriter>> {
        if let Some(user) = self.users.get(&client_id) {
            Some(user.0.clone())
        } else {
            None
        }
    }

    fn delete_user(&mut self, client_id: String) {
        if self.users.remove(&client_id).is_none() {
            println!("Unexpected error");
        }
    }

    fn get_sender(&self, client_id: String) -> Option<Sender<ChannelPublisherWriter>> {
        if let Some(publisher_writer) = self.find_user(client_id) {
            Some(publisher_writer)
        } else {
            println!("Unexpected error: user not found in user_manager");
            None
        }
    }

    // TODO: agregar tests que aseguren que funciona
    fn disconect(&mut self, client_id: String) {
        if let Some(user) = self.users.get(&client_id) {
            let publisher_writer_cloned = user.0.clone();
            let persistent_session = user.1;
            publisher_writer_cloned
                .send((DisconectPublisherSubscriber, None, None))
                .unwrap();
            self.users
                .insert(client_id, (publisher_writer_cloned, persistent_session));
        }
    }
}
