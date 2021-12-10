use crate::enums::topic_manager::topic_message::TypeMessage;
use crate::enums::user_manager::user_manager_action::UserManagerAction;
use crate::logs::logger::Logger;
use crate::packets::packet_manager::PacketManager;
use crate::stream::stream_handler::Stream;
use crate::stream::stream_handler::StreamAction::ReadStream;
use crate::stream::stream_handler::StreamType;
use std::sync::mpsc::Receiver;

use std::net::TcpListener;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

pub fn handle_new_client(
    mut logger: Logger,
    sender_stream: Sender<StreamType>,
    sender_topic_manager: Sender<TypeMessage>,
    sender_user_manager: Sender<UserManagerAction>,
) {
    let mut packet_factory = PacketManager::init(
        sender_user_manager,
        sender_stream.clone(),
        sender_topic_manager,
        logger.clone(),
    );

    let (sender, receiver): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();

    loop {
        let message_sent =
            sender_stream
                .clone()
                .send((ReadStream, None, Some(sender.clone()), None));

        if let Err(msg) = message_sent {
            logger.info(format!("Error receiving a message: {}", msg));
        } else if let Ok(packet) = receiver.recv() {
            logger.info(format!("Packet received: {:?}", packet));
            let packet_u8: &[u8] = &packet;
            if let Err(err) = packet_factory.process_message(packet_u8) {
                logger.info(format!("Error processing the packet received: {}", err));
                break;
            }
        } else if let Err(err) = receiver.recv() {
            logger.info(format!("Error reading the packet received: {}", err));
            break;
        }
    }
}

pub fn run_server(
    listener: &TcpListener,
    mut logger: Logger,
    sender_topic_manager: Sender<TypeMessage>,
    sender_user_manager: Sender<UserManagerAction>,
) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                logger.info(format!("New connection: {}", stream.peer_addr().unwrap()));

                let logger_clone = logger.clone();
                let sender_tm_cloned = sender_topic_manager.clone();
                let sender_um_cloned = sender_user_manager.clone();
                let sender_stream = Stream::init(stream, logger.clone());

                thread::spawn(move || {
                    handle_new_client(
                        logger_clone,
                        sender_stream,
                        sender_tm_cloned,
                        sender_um_cloned,
                    );
                });
            }
            Err(e) => {
                logger.error(format!("Error on connection: {}", e));
            }
        }
    }
}
