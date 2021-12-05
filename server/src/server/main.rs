use crate::logs::logger::Logger;
use crate::paquetes::packet_manager::PacketManager;
use crate::paquetes::publisher_suscriber::PublisherSuscriber;
use crate::server::types::TypeToCloseConnection;
use crate::stream::stream_handler::Stream;
use crate::stream::stream_handler::StreamAction::CloseConnectionStream;
use crate::stream::stream_handler::StreamAction::ReadStream;
use crate::stream::stream_handler::StreamType;
use crate::usermanager::user_manager_action::UserManagerAction::DisconectUserManager;
use crate::usermanager::user_manager_types::ChannelUserManager;
use std::sync::mpsc::Receiver;

use std::net::TcpListener;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

pub fn handle_new_client(
    mut logger: Logger,
    sender_stream: Sender<StreamType>,
    sender_topic_manager: Sender<PublisherSuscriber>,
    sender_user_manager: Sender<ChannelUserManager>,
) {
    let (sender_to_close_connection, receiver_to_close_connection): (
        Sender<TypeToCloseConnection>,
        Receiver<TypeToCloseConnection>,
    ) = mpsc::channel();

    let cloned_sender_user_manager = sender_user_manager.clone();
    let cloned_sender_stream = sender_stream.clone();

    let mut prueba_logger = logger.clone();

    let _t = thread::spawn(move || {
        for receive_message in receiver_to_close_connection {
            prueba_logger.info("has gotten a event for connection closing".to_string());
            let (client_id, _) = receive_message;
            let event_user_manager_disconection = cloned_sender_user_manager.send((
                DisconectUserManager,
                client_id.to_owned(),
                None,
                None,
                None,
            ));

            prueba_logger.info("has disconected the user manager".to_string());

            if let Err(err_sender_um) = event_user_manager_disconection {
                prueba_logger.info("has failed the user manager disconnection".to_string());
                println!("Unexpected err: {}", err_sender_um);
            } else {
                prueba_logger.info("has sent a event to stream handler".to_string());
                cloned_sender_stream
                    .send((CloseConnectionStream, None, None))
                    .unwrap();

                prueba_logger.info("has closed the connection correclty".to_string());
            }
        }
    });

    let mut packet_factory = PacketManager::init(
        sender_user_manager,
        sender_to_close_connection,
        sender_stream.clone(),
        sender_topic_manager,
        logger.clone(),
    );

    let (sender, receiver): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();

    loop {
        let message_sent = sender_stream
            .clone()
            .send((ReadStream, None, Some(sender.clone())));

        // let mut error = false;

        if let Err(msg) = message_sent {
            logger.info(format!("Error receiving a message: {}", msg));
        } else if let Ok(packet) = receiver.recv() {
            logger.info(format!("packet received: {:?}", packet));
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
    sender_topic_manager: Sender<PublisherSuscriber>,
    sender_user_manager: Sender<ChannelUserManager>,
) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                logger.info(format!("New connection: {}", stream.peer_addr().unwrap()));

                let logger_clone = logger.clone();
                let sender_tm_cloned = sender_topic_manager.clone();
                let sender_um_cloned = sender_user_manager.clone();
                let sender_stream = Stream::init(stream);

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
