use std::io::Read;
use std::{net::TcpStream, io::Write};

use crate::integration::setup::ServerTest;


#[test]
fn should_disconnect_when_publish_message_with_random_qos_value() {
    let server = ServerTest::start("0.0.0.0:1454".to_string());

    let mut subscriber_stream = TcpStream::connect("0.0.0.0:1454".to_string()).unwrap();
    let mut publisher_stream = TcpStream::connect("0.0.0.0:1454".to_string()).unwrap();

    let mut data = vec![0; 100];

    let subscriber_connect_bytes = [
        0x10, // packet type
        0x0E, // remaining length
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, //mqtt
        0x04, // protocol name
        0x00, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x62, 0x6F, // client identifier
    ];

    let publisher_connect_bytes = [
        0x10, // packet type
        0x0E, // remaining length
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, //mqtt
        0x04, // protocol name
        0x00, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x63, 0x61, // client identifier
    ];

    subscriber_stream.write(&subscriber_connect_bytes).unwrap();
    publisher_stream.write(&publisher_connect_bytes).unwrap();

    match subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
        }
        _ => {
            panic!();
        }
    }

    data = vec![0; 100];
    match publisher_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
        }
        _ => {
            panic!();
        }
    }

    let subscribe_bytes = [
        0x82, // packet type
        0x08, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
        0x00, 0x03, 0x61, 0x2F, 0x62, 0x00, // payload MQTT como mensaje + qos
    ];

    subscriber_stream.write(&subscribe_bytes).unwrap();

    data = vec![0; 100];
    match subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x00]);
        }
        _ => {
            panic!();
        }
    }

    let publish_bytes = [
        0x36, // tiene la información del packet type 0011, dup flag + qos flag + retain flag
        0x0C, // remaining length
        0x00, 0x03, 0x61, 0x2F, 0x62, // topic name
        0x00, 0x0A, // packet identifier
        0x00, 0x03, 0x61, 0x2F, 0x61, // payload
    ];
    publisher_stream.write(&publish_bytes).unwrap();

    data = vec![0; 100];
    match subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], []);
        }
        _ => {
            panic!();
        }
    }

    server.shutdown().unwrap();
}