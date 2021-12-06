use crate::integration::setup::ServerTest;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::{thread, time};

#[test]
fn should_publish_message_with_qos_0_01() {
    let server = ServerTest::start("0.0.0.0:1893".to_string());

    let mut subscriber_stream = TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();
    let mut publisher_stream = TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();

    let mut data = vec![0; 100];

    let subscriber_connect_bytes = [
        0x10, // packet type
        0x0E, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
        0x04, // protocol name
        0x00, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x62, 0x6F, // client identifier
    ];

    let publisher_connect_bytes = [
        0x10, // packet type
        0x0E, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
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
        0x30, // tiene la información del packet type 0011, dup flag + qos flag + retain flag
        0x0C, // remaining length
        0x00, 0x03, 0x61, 0x2F, 0x62, // topic name
        0x00, 0x0A, // packet identifier
        0x00, 0x03, 0x61, 0x2F, 0x61, // payload
    ];
    publisher_stream.write(&publish_bytes).unwrap();

    data = vec![0; 100];
    match subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], publish_bytes);
        }
        _ => {
            panic!();
        }
    }

    server.shutdown().unwrap();
}

#[test]
fn should_publish_message_with_qos_1() {
    let server = ServerTest::start("0.0.0.0:1894".to_string());

    let mut subscriber_stream = TcpStream::connect("0.0.0.0:1894".to_string()).unwrap();
    let mut publisher_stream = TcpStream::connect("0.0.0.0:1894".to_string()).unwrap();

    let mut data = vec![0; 100];

    let subscriber_connect_bytes = [
        0x10, // packet type
        0x0E, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
        0x04, // protocol name
        0x00, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x62, 0x6F, // client identifier
    ];

    let publisher_connect_bytes = [
        0x10, // packet type
        0x0E, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
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
        0x00, 0x03, 0x61, 0x2F, 0x62, 0x01, // payload MQTT como mensaje + qos
    ];

    subscriber_stream.write(&subscribe_bytes).unwrap();

    data = vec![0; 100];
    match subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x01]);
        }
        _ => {
            panic!();
        }
    }

    let publish_bytes = [
        0x32, // tiene la información del packet type 0011, dup flag + qos flag + retain flag
        0x0C, // remaining length
        0x00, 0x03, 0x61, 0x2F, 0x62, // topic name
        0x00, 0x0A, // packet identifier
        0x00, 0x03, 0x61, 0x2F, 0x61, // payload
    ];
    publisher_stream.write(&publish_bytes).unwrap();

    data = vec![0; 100];
    match publisher_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x40, 0x01, 0x00, 0x0A]);
        }
        _ => {
            panic!();
        }
    }

    data = vec![0; 100];
    match subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], publish_bytes);
        }
        _ => {
            panic!();
        }
    }

    server.shutdown().unwrap();
}

#[test]
fn should_publish_message_with_qos_1_twice() {
    let server = ServerTest::start("0.0.0.0:1894".to_string());

    let mut subscriber_stream = TcpStream::connect("0.0.0.0:1894".to_string()).unwrap();
    let mut publisher_stream = TcpStream::connect("0.0.0.0:1894".to_string()).unwrap();

    let mut data = vec![0; 100];

    // CONECTA AMBOS USERS
    let subscriber_connect_bytes = [
        0x10, // packet type
        0x0E, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
        0x04, // protocol name
        0x00, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x62, 0x6F, // client identifier
    ];

    let publisher_connect_bytes = [
        0x10, // packet type
        0x0E, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
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

    // SUBSCRIBE
    let subscribe_bytes = [
        0x82, // packet type
        0x08, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
        0x00, 0x03, 0x61, 0x2F, 0x62, 0x01, // payload MQTT como mensaje + qos
    ];

    subscriber_stream.write(&subscribe_bytes).unwrap();

    data = vec![0; 100];
    match subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x01]);
        }
        _ => {
            panic!();
        }
    }

    // PUBLICA
    let publish_bytes = [
        0x32, // tiene la información del packet type 0011, dup flag + qos flag + retain flag
        0x0C, // remaining length
        0x00, 0x03, 0x61, 0x2F, 0x62, // topic name
        0x00, 0x0A, // packet identifier
        0x00, 0x03, 0x61, 0x2F, 0x61, // payload
    ];
    publisher_stream.write(&publish_bytes).unwrap();

    thread::sleep(time::Duration::from_millis(100));

    data = vec![0; 100];
    match publisher_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x40, 0x01, 0x00, 0x0A]);
        }
        _ => {
            panic!();
        }
    }

    data = vec![0; 100];
    match subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], publish_bytes);
        }
        _ => {
            panic!();
        }
    }

    // PUBLICA OTRA VEZ
    publisher_stream.write(&publish_bytes).unwrap();

    data = vec![0; 100];
    match publisher_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x40, 0x01, 0x00, 0x0A]);
        }
        _ => {
            panic!();
        }
    }

    data = vec![0; 100];
    match subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], publish_bytes);
        }
        _ => {
            panic!();
        }
    }

    server.shutdown().unwrap();
}
