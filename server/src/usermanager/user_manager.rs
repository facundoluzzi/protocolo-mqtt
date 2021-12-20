use crate::enums::publisher_writter::reconnect_stream::ReconnectStream;
use crate::enums::publisher_writter::stop_publish_to_stream::StopPublishToStream;
use crate::enums::topic_manager::topic_message::TypeMessage;
use crate::enums::topic_manager::unsubscriberall::UnsubscriberAll;
use crate::enums::user_manager::add_user_manager::AddUserManager;
use crate::enums::user_manager::disconnect_user_manager::DisconnectUserManager;
use crate::enums::user_manager::publish_message_user_manager::PublishMessageUserManager;
use crate::enums::user_manager::stop_publish_user_manager::StopPublish;
use crate::enums::user_manager::user_manager_action::UserManagerAction;
use crate::enums::user_manager::valid_client_id_user_manager::ValidClientIdUserManager;
use crate::packets::publish::Publish;
use crate::stream::stream_handler::StreamType;
use crate::topic::publisher_writer::PublisherWriter;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::{collections::HashMap, sync::mpsc::Sender};

pub struct UserManager {
    users: HashMap<String, (PublisherWriter, bool, Option<Publish>)>,
    sender_topic_manager: Sender<TypeMessage>,
}

impl UserManager {
    /// Constructor del struct. Lanza un thread para quedarse escuchando por eventos.
    /// Los eventos pueden ser AddUserManager, DisconnectUserManager, PublishMessageUserManager, StopToPublishUserManager
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
                        user_manager.process_new_connection(user);
                    }
                    UserManagerAction::DisconnectUserManager(user) => {
                        user_manager.process_disconnect(user);
                    }
                    UserManagerAction::PublishMessageUserManager(user) => {
                        user_manager.process_publish_message(user);
                    }
                    UserManagerAction::StopPublishUserManager(user) => {
                        user_manager.process_stop_publish_message(user);
                    }
                    UserManagerAction::ValidClientId(user) => {
                        user_manager.process_is_valid_client_id(user);
                    }
                }
            }
        });
        sender_user_manager
    }

    fn process_is_valid_client_id(&mut self, user_client: ValidClientIdUserManager) {
        let user = self.find_user(user_client.get_client_id());
        if let Some(user_found) = user {
            let (publisher_writer, _clean_session, _publish) = user_found;
            if let Err(err) = user_client
                .get_sender()
                .send(publisher_writer.is_disconnected())
            {
                println!("Unexpected error: {}", err);
            }
        } else if let Err(err) = user_client.get_sender().send(true) {
            println!("Unexpected error: {}", err);
        }
    }

    fn process_new_connection(&mut self, user: AddUserManager) {
        let sender_stream = user.get_sender_stream();
        let mut publish: Option<Publish> = None;
        if let Some(user_on_hash) = self.users.remove(&user.get_client_id()) {
            let reconnect = ReconnectStream::init(sender_stream.clone());
            let mut publisher_writer = user_on_hash.0;
            publisher_writer.reconnect(reconnect);

            if let Some(message) = user.get_will_message() {
                publish = Some(self.generate_will_publish(
                    user.get_will_topic(),
                    message,
                    user.get_will_qos(),
                    user.get_will_retain_message(),
                ));
            }
            self.add(
                user.get_client_id(),
                sender_stream,
                user.get_clean_session(),
                publish,
            );
        } else {
            if let Some(message) = user.get_will_message() {
                publish = Some(self.generate_will_publish(
                    user.get_will_topic(),
                    message,
                    user.get_will_qos(),
                    user.get_will_retain_message(),
                ));
            }
            self.add(
                user.get_client_id(),
                sender_stream,
                user.get_clean_session(),
                publish,
            )
        }
    }

    fn process_publish_message(&mut self, user: PublishMessageUserManager) {
        let client_id = user.get_client_id();
        let message = user.get_message();
        if let Some(find_user_result) = self.find_user(client_id.to_string()) {
            let (mut publisher_writer, clean_session, publish) = find_user_result;
            publisher_writer.publish_message(message);
            self.users
                .insert(client_id, (publisher_writer, clean_session, publish));
        }
    }

    fn process_stop_publish_message(&mut self, user: StopPublish) {
        let client_id = user.get_client_id();
        let packet_identifier = user.get_packet_identifier();

        if let Some(find_user_result) = self.find_user(client_id.to_string()) {
            let (mut publisher_writer, clean_session, publish) = find_user_result;
            let stop_publish = StopPublishToStream::init(packet_identifier.to_vec());
            publisher_writer.remove(stop_publish);
            self.users
                .insert(client_id, (publisher_writer, clean_session, publish));
        }
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

    fn generate_publish_bytes(first_b: u8, topic_b: &[u8], payload: &[u8]) -> Vec<u8> {
        let remaining_length = (6 + payload.len() + topic_b.len()) as u8;
        let mut publish: Vec<u8> = vec![first_b, remaining_length, 0x00, topic_b.len() as u8];
        for topic in topic_b {
            publish.push(*topic);
        }
        let bytes_to_concat = [0x00, 0x0A, 0x00, payload.len() as u8];
        publish = [publish.to_vec(), bytes_to_concat.to_vec()].concat();
        for message in payload {
            publish.push(*message);
        }

        publish
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
        let publish = UserManager::generate_publish_bytes(publish_bytes, topic_bytes, payload);
        if let Ok(packet) = Publish::init(&publish) {
            packet
        } else {
            panic!("Unexpected error: publish packet couldn't be created");
        }
    }

    fn find_user(&self, client_id: String) -> Option<(PublisherWriter, bool, Option<Publish>)> {
        self.users
            .get(&client_id)
            .map(|t| (t.0.clone(), t.1, t.2.clone()))
    }

    fn process_disconnect(&mut self, user: DisconnectUserManager) {
        let client_id = user.get_client_id();
        let disconnection_ungracefully = user.get_disconnection_type();
        let (publisher_writer, clean_session, publish): (
            Option<PublisherWriter>,
            Option<bool>,
            Option<Publish>,
        ) = match self.users.get(&client_id) {
            Some(user) => (Some(user.0.clone()), Some(user.1), user.2.clone()),
            None => (None, None, None),
        };

        if disconnection_ungracefully {
            self.publish_last_will_message(user);
        }

        if let Some(clean_session) = clean_session {
            self.process_clean_session(client_id, clean_session, publisher_writer, publish);
        }
    }

    fn process_clean_session(
        &mut self,
        client_id: String,
        clean_session: bool,
        publisher_writer: Option<PublisherWriter>,
        publish: Option<Publish>,
    ) {
        if clean_session {
            if self.users.remove(&client_id).is_none() {
                println!("Unexpected error");
            }
            let unsubscriber_all = UnsubscriberAll::init(client_id);
            if let Err(err) = self
                .sender_topic_manager
                .send(TypeMessage::UnsubscriberAll(unsubscriber_all))
            {
                println!("{}", err.to_string());
            }
        } else if let Some(mut pub_writer) = publisher_writer {
            pub_writer.disconnect();
            self.users
                .insert(client_id, (pub_writer, clean_session, publish));
        }
    }

    fn publish_last_will_message(&mut self, user: DisconnectUserManager) {
        let client_id = user.get_client_id();
        if let Some(user) = self.users.get(&client_id) {
            if let Some(publish) = &user.2 {
                if let Err(err) =
                    publish.send_message(self.sender_topic_manager.clone(), client_id.to_owned())
                {
                    println!("{}", err);
                }
            }
        }

        let (publisher_writer, clean_session): (Option<PublisherWriter>, bool) =
            match self.users.get(&client_id) {
                Some(user) => (Some(user.0.clone()), user.1),
                None => (None, false),
            };
        if let Some(publisher_writer_found) = publisher_writer {
            self.users
                .insert(client_id, (publisher_writer_found, clean_session, None));
        }
    }
}
