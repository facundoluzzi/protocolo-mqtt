use crate::helper::user_manager::UserManager;
use crate::logs::logger::Logger;
use crate::paquetes::packet_manager::PacketManager;
use crate::paquetes::publisher_suscriber::PublisherSuscriber;
use crate::helper::remaining_length::save_remaining_length;

use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::thread;

fn handle_new_client(
    mut stream: TcpStream,
    mut logger: Logger,
    publish_subscriber_sender: &Sender<PublisherSuscriber>,
    mut user_manager: UserManager,
) {
    // TODO: revisar el largo
    let mut data = [0_u8; 5];
    // TODO: ver que onda el while
    let mut packet_factory = PacketManager::init();
    let mut total_data: Vec<u8> = Vec::new();

    let mut is_first_byte = true;
    let mut packet_length = 0;
    let mut readed_bytes = 0;

    while match stream.read(&mut data) {
        Ok(size) => {
            if is_first_byte && size != 0 {
                let (_readed_bytes, _packet_length) = save_remaining_length(&data[1..size]).unwrap();
                packet_length = _packet_length;
                readed_bytes = _readed_bytes;

                is_first_byte = false;
                total_data = [total_data, data.to_vec()].concat();
                true
            } else if !is_first_byte && size != 0 {
                total_data = [total_data, data.to_vec()].concat();

                // readed bytes son los bytes leídos en el remaining_length y el +1 es el primer byte
                if total_data.len() >= packet_length + readed_bytes + 1 {
                    logger.info(format!("Received from client {:?}", &total_data[0..packet_length + readed_bytes + 1]));
                    is_first_byte = true;
                    packet_factory.process_message(
                        &total_data[0..packet_length + readed_bytes + 1],
                        &stream,
                        publish_subscriber_sender,
                        &mut user_manager,
                    );
                    total_data = Vec::new();
                }
                true
            } else {
                false
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
    publish_subscriber_sender: Sender<PublisherSuscriber>,
    user_manager: UserManager,
) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                logger.info(format!("New connection: {}", stream.peer_addr().unwrap()));
                let logger_clone = logger.clone();
                let publish_subscriber_sender_cloned = publish_subscriber_sender.clone();
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
