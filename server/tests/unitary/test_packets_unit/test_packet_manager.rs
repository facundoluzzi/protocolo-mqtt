use server::logs::logger::Logger;
use server::packets::packet_manager::PacketManager;
use server::stream::stream_handler::Stream;
use server::topic::topic_manager::TopicManager;
use server::usermanager::user_manager::UserManager;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn setup() {
    match TcpListener::bind("0.0.0.0:1883") {
        Ok(listener) => {
            thread::spawn(move || for _i in listener.incoming() {});
            thread::sleep(Duration::from_millis(100));
        }
        Err(_) => {}
    }
}

#[test]
fn send_connect_should_select_connect_type() {
    let bytes = [
        0x10, 0x0E, 0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, 0x04, 0x00, 0x00, 0x0B, 0x00, 0x02, 0x00,
        0x00,
    ];
    let connect_packet = PacketManager::get_control_packet_type(bytes[0]);
    assert_eq!(connect_packet, 1);
}

#[test]
fn send_publish_should_select_publish_type() {
    let bytes = [
        0x30, 0x0A, 0x00, 0x05, 0x54, 0x4F, 0x50, 0x49, 0x43, 0x00, 0x06, 0x54,
    ];
    let publish_packet = PacketManager::get_control_packet_type(bytes[0]);
    assert_eq!(publish_packet, 3);
}

#[test]
fn send_subscribe_should_select_subscribe_type() {
    let bytes = [
        0x80, 0x0A, 0x00, 0x05, 0x54, 0x4F, 0x50, 0x49, 0x43, 0x00, 0x06, 0x54,
    ];
    let subscribe_packet = PacketManager::get_control_packet_type(bytes[0]);
    assert_eq!(subscribe_packet, 8);
}

#[test]
fn send_an_invalid_packet_type_should_fail() {
    let bytes = [
        0x00, 0x0A, 0x00, 0x05, 0x54, 0x4F, 0x50, 0x49, 0x43, 0x00, 0x06, 0x54,
    ];

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

    if let Err(err) = packet_manager.process_message(&bytes) {
        assert_eq!(
            err.to_string(),
            "Unexpecter error: packet received don't match with another packet".to_string()
        );
    } else {
        panic!("Should fail");
    }
}

#[test]
fn send_an_empty_packet_should_fail() {
    let bytes = [];

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

    if let Err(err) = packet_manager.process_message(&bytes) {
        assert_eq!(
            err.to_string(),
            "Unexpecter error: packet malformed".to_string()
        );
    } else {
        panic!("Should fail");
    }
}
