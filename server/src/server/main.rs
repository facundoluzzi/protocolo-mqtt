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

fn process_packets(
    mut packet_manager: PacketManager,
    mut logger: Logger,
) -> Result<String, String> {
    let (sender, receiver): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
    let sender_stream = packet_manager.get_sender_stream();

    loop {
        receive_message(sender_stream.clone(), sender.clone())?;
        match receiver.recv() {
            Ok(packet) => {
                let result_process_message = packet_manager.process_message(&packet);
                if let Err(err) = result_process_message {
                    return Err(format!("Error processing the packet received: {}", err));
                } else {
                    logger.info(format!("Packet received: {:?}", packet));
                }
            }
            Err(err) => return Err(format!("Error reading the packet received: {}", err)),
        }
    }
}

fn receive_message(
    sender_stream: Sender<StreamType>,
    sender: Sender<Vec<u8>>,
) -> Result<(), String> {
    let sender_result = sender_stream.send((ReadStream, None, Some(sender), None));
    match sender_result {
        Ok(()) => Ok(()),
        Err(err) => Err(format!("Error receiving a message: {}", err)),
    }
}

/// maneja los nuevos clientes
pub fn handle_new_client(
    mut logger: Logger,
    sender_stream: Sender<StreamType>,
    sender_topic_manager: Sender<TypeMessage>,
    sender_user_manager: Sender<UserManagerAction>,
) {
    let packet_manager = PacketManager::init(
        sender_user_manager,
        sender_stream,
        sender_topic_manager,
        logger.clone(),
    );

    if let Err(err) = process_packets(packet_manager, logger.clone()) {
        logger.info(err);
    }
}

/// corre el servidor
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
                let sender_stream = if let Ok(stream) = Stream::init(stream, logger.clone()) {
                    stream
                } else {
                    panic!("Unexpected error: Stream can't be created");
                };

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
