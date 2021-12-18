use std::io::Read;
use std::{net::TcpStream, io::Write};

use crate::integration::setup::ServerTest;

#[test]
fn should_disconnect_when_unsubscribe_have_bytes_on_reserved_bytes() {
    let server = ServerTest::start("0.0.0.0:1543".to_string());
    let mut stream = TcpStream::connect("0.0.0.0:1543".to_string()).unwrap();

    let connect_bytes = [
        0x10, // packet type
        0x10, // remaining length
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, //mqtt
        0x04, // protocol name
        0x00, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x04, 0x62, 0x6F, 0x63, 0x61, // client identifier
    ];

    stream.write(&connect_bytes).unwrap();

    let mut data = vec![0; 1000];
    stream.read(&mut data).unwrap();

    let subscribe_bytes = [
        0x82, // packet type
        0x09, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, 0x00, // payload MQTT como mensaje + qos
    ];
    stream.write(&subscribe_bytes).unwrap();

    stream.read(&mut data).unwrap();

    let unsubscribe_bytes = [
        0xA3, // packet type
        0x08, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, // payload MQTT como mensaje
    ];
    stream.write(&unsubscribe_bytes).unwrap();

    match stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], []);
        }
        _ => {
            panic!();
        }
    }

    server.shutdown().unwrap();
}

#[test]
fn should_disconnect_when_unsubscribe_with_no_topics() {
    let server = ServerTest::start("0.0.0.0:1541".to_string());
    let mut stream = TcpStream::connect("0.0.0.0:1541".to_string()).unwrap();

    let connect_bytes = [
        0x10, // packet type
        0x10, // remaining length
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, //mqtt
        0x04, // protocol name
        0x00, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x04, 0x62, 0x6F, 0x63, 0x61, // client identifier
    ];

    stream.write(&connect_bytes).unwrap();

    let mut data = vec![0; 1000];
    stream.read(&mut data).unwrap();

    let subscribe_bytes = [
        0x82, // packet type
        0x09, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, 0x00, // payload MQTT como mensaje + qos
    ];
    stream.write(&subscribe_bytes).unwrap();

    stream.read(&mut data).unwrap();

    let unsubscribe_bytes = [
        0xA2, // packet type
        0x02, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
    ];
    stream.write(&unsubscribe_bytes).unwrap();

    match stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], []);
        }
        _ => {
            panic!();
        }
    }

    server.shutdown().unwrap();
}