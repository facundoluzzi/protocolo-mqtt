use crate::helper::user_manager::UserManager;
use crate::logs::logger::Logger;
use crate::paquetes::packet_manager::PacketManager;
use crate::paquetes::publisher_suscriber::PublisherSuscriber;
use crate::topics::topic_manager::TopicManager;

use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::thread;

fn handle_new_client(
    mut stream: TcpStream,
    mut logger: Logger,
    publish_subscriber_sender: &Sender<PublisherSuscriber>,
    user_manager: UserManager,
) {
    // TODO: revisar el largo
    let mut data = [0_u8; 100];
    // TODO: ver que onda el while
    let packet_factory = PacketManager::init();
    while match stream.read(&mut data) {
        Ok(size) => {
            if size == 0 {
                false
            } else {
                logger.info(format!("Received from client {:?}", &data[0..size]));
                packet_factory.process_message(
                    &data[0..size],
                    &stream,
                    publish_subscriber_sender,
                    user_manager.clone(),
                );
                true
            }
        }
        Err(_) => {
            logger.error(format!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            ));
            stream.shutdown(Shutdown::Both).unwrap();
            true
        }
    } {}
}

pub fn run_server(
    listener: &TcpListener,
    mut logger: Logger,
    publish_subscriber_sender: TopicManager,
    user_manager: UserManager,
) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                logger.info(format!("New connection: {}", stream.peer_addr().unwrap()));
                let logger_clone = logger.clone();
                let publish_subscriber_sender_cloned = publish_subscriber_sender.get_sender();
                let user_manager_cloned = user_manager.clone();
                thread::spawn(move || {
                    handle_new_client(
                        stream,
                        logger_clone,
                        &publish_subscriber_sender_cloned,
                        user_manager_cloned,
                    );
                });
            }
            Err(e) => {
                logger.error(format!("Error on connection: {}", e));
            }
        }
    }
}
