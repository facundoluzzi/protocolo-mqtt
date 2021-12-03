use crate::stream::stream_handler::StreamType;
use crate::topics::publisher_writer::PublisherWriter;
use crate::usermanager::user_manager_action::UserManagerAction::{
    AddUserManager, DeleteUserManager, DisconectUserManager, GetSenderUserManager,
};
use crate::usermanager::user_manager_types::ChannelUserManager;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::{collections::HashMap, sync::mpsc::Sender};

pub struct UserManager {
    users: HashMap<String, (PublisherWriter, bool)>,
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
                match receive {
                    (AddUserManager, client_id, Some(sender_stream), Some(clean_session), None) => {
                        if let Some(mut usuario) = user_manager.find_user(client_id.to_string()) {
                            usuario.reconnect(sender_stream);
                        } else {
                            user_manager.add(
                                client_id.to_string(),
                                sender_stream.clone(),
                                clean_session,
                            );
                        };
                    }
                    (DeleteUserManager, client_id, None, None, None) => {
                        user_manager.delete_user(client_id);
                    }
                    (DisconectUserManager, client_id, None, None, None) => {
                        user_manager.disconect(client_id);
                    }
                    (GetSenderUserManager, client_id, None, None, Some(sender)) => {
                        let sender_for_publish = user_manager.get_sender(client_id);
                        sender.send(sender_for_publish).unwrap();
                    }
                    _ => {}
                }
            }
        });

        sender_user_manager
    }

    fn add(&mut self, client_id: String, stream: Sender<StreamType>, clean_session: bool) {
        let publisher_writer = PublisherWriter::init(stream, client_id.to_owned());
        self.users
            .insert(client_id, (publisher_writer, clean_session));
    }

    fn find_user(&self, client_id: String) -> Option<PublisherWriter> {
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

    fn get_sender(&self, client_id: String) -> Sender<String> {
        let publisher_writer = self.find_user(client_id).unwrap();
        publisher_writer.get_sender()
    }

    // TODO: agregar tests que aseguren que funciona
    fn disconect(&mut self, client_id: String) {
        if let Some(user) = self.users.get(&client_id) {
            let mut publisher_writer_cloned = user.0.clone();
            let clean_session = user.1;
            if clean_session == true{
                self.delete_user(client_id.to_owned())
            } else{
                publisher_writer_cloned.disconect();
            self.users
                .insert(client_id, (publisher_writer_cloned, false));
            }
            
        }
    }
}
