use server::logs::logger::Logger;
use server::packets::connect::Connect;
use server::packets::packet_manager::PacketManager;
use server::stream::stream_handler::Stream;
use server::topic::topic_manager::TopicManager;
use server::usermanager::user_manager::UserManager;
use std::net::TcpListener;
use std::net::TcpStream;
use std::{thread, time};

fn setup() {
    match TcpListener::bind("0.0.0.0:1883") {
        Ok(listener) => {
            thread::spawn(move || for _i in listener.incoming() {});
            thread::sleep(time::Duration::from_millis(100));
        }
        Err(_) => {}
    }
}

#[test]
fn should_connect_successfully() {
    setup();
    let stream = TcpStream::connect("localhost:1883").unwrap();
    let logger = Logger::new("connect-unit-test.log".to_string()).unwrap();
    let sender_topic_manager = TopicManager::init();
    let sender_user_manager = UserManager::init(sender_topic_manager.clone());
    let sender_stream = Stream::init(stream, logger.clone());

    let mut packet_manager = PacketManager::init(
        sender_user_manager,
        sender_stream.unwrap(),
        sender_topic_manager,
        logger,
    );

    let connect_bytes = [
        0x10, // packet type
        0x12, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
        0x04, // protocol name
        0x00, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, // client identifier
    ];
    Connect::process_message(&connect_bytes, &mut packet_manager).unwrap();
    assert_eq!(packet_manager.is_disconnected(), false);
}

#[test]
fn should_throw_error_when_a_user_try_to_connect() {
    setup();
    let stream = TcpStream::connect("localhost:1883").unwrap();
    let logger = Logger::new("connect-unit-test.log".to_string()).unwrap();
    let sender_topic_manager = TopicManager::init();
    let sender_user_manager = UserManager::init(sender_topic_manager.clone());
    let sender_stream = Stream::init(stream, logger.clone());

    let mut packet_manager = PacketManager::init(
        sender_user_manager,
        sender_stream.unwrap(),
        sender_topic_manager,
        logger,
    );

    packet_manager.connect();

    let connect_bytes = [
        0x10, // packet type
        0x12, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
        0x04, // protocol name
        0x00, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, // client identifier
    ];

    if let Err(err) = Connect::process_message(&connect_bytes, &mut packet_manager) {
        assert_eq!(err, "Client is already connected".to_string());
    } else {
        panic!("should be error");
    }
}

#[test]
fn should_throw_an_error_when_the_packet_has_one_byte() {
    setup();
    let stream = TcpStream::connect("localhost:1883").unwrap();
    let logger = Logger::new("connect-unit-test.log".to_string()).unwrap();
    let sender_topic_manager = TopicManager::init();
    let sender_user_manager = UserManager::init(sender_topic_manager.clone());
    let sender_stream = Stream::init(stream, logger.clone());

    let mut packet_manager = PacketManager::init(
        sender_user_manager,
        sender_stream.unwrap(),
        sender_topic_manager,
        logger,
    );

    let connect_bytes = [0x10];

    if let Err(err) = Connect::process_message(&connect_bytes, &mut packet_manager) {
        assert_eq!(err, "packet malformed".to_string());
    } else {
        panic!("should be error");
    }
}

#[test]
fn should_throw_an_error_when_the_packet_has_an_invalid_rem_length() {
    setup();
    let stream = TcpStream::connect("localhost:1883").unwrap();
    let logger = Logger::new("connect-unit-test.log".to_string()).unwrap();
    let sender_topic_manager = TopicManager::init();
    let sender_user_manager = UserManager::init(sender_topic_manager.clone());
    let sender_stream = Stream::init(stream, logger.clone());

    let mut packet_manager = PacketManager::init(
        sender_user_manager,
        sender_stream.unwrap(),
        sender_topic_manager,
        logger,
    );

    let connect_bytes = [
        0x10, // packet type
        0x15, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
        0x04, // protocol name
        0x00, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, // client identifier
    ];

    if let Err(err) = Connect::process_message(&connect_bytes, &mut packet_manager) {
        assert_eq!(err, "packet malformed".to_string());
    } else {
        panic!("should be error");
    }
}
