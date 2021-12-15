use crate::integration::setup::ServerTest;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn should_publish_message_with_qos_0_01() {
    let server = ServerTest::start("0.0.0.0:2532".to_string());

    let mut subscriber_stream = TcpStream::connect("0.0.0.0:2532".to_string()).unwrap();
    let mut publisher_stream = TcpStream::connect("0.0.0.0:2532".to_string()).unwrap();

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
            assert_eq!(data[0..size], [0x40, 0x02, 0x00, 0x0A]);
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
    let server = ServerTest::start("0.0.0.0:2537".to_string());

    let mut subscriber_stream = TcpStream::connect("0.0.0.0:2537".to_string()).unwrap();
    let mut publisher_stream = TcpStream::connect("0.0.0.0:2537".to_string()).unwrap();

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

    data = vec![0; 100];
    match publisher_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x40, 0x02, 0x00, 0x0A]);
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
            assert_eq!(data[0..size], [0x40, 0x02, 0x00, 0x0A]);
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
fn should_publish_message_with_both_qos() {
    let server = ServerTest::start("0.0.0.0:2538".to_string());

    let mut subscriber_stream = TcpStream::connect("0.0.0.0:2538".to_string()).unwrap();
    let mut publisher_stream = TcpStream::connect("0.0.0.0:2538".to_string()).unwrap();

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

    // PUBLICA con qos 0
    let publish_bytes_qos_0 = [
        0x30, // tiene la información del packet type 0011, dup flag + qos flag + retain flag
        0x0C, // remaining length
        0x00, 0x03, 0x61, 0x2F, 0x62, // topic name
        0x00, 0x0A, // packet identifier
        0x00, 0x03, 0x61, 0x2F, 0x61, // payload
    ];

    publisher_stream.write(&publish_bytes_qos_0).unwrap();

    data = vec![0; 100];
    match subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], publish_bytes_qos_0);
        }
        _ => {
            panic!();
        }
    }

    // PUBLICA con qos 1
    let publish_bytes_qos_1 = [
        0x32, // tiene la información del packet type 0011, dup flag + qos flag + retain flag
        0x0C, // remaining length
        0x00, 0x03, 0x61, 0x2F, 0x62, // topic name
        0x00, 0x0A, // packet identifier
        0x00, 0x03, 0x61, 0x2F, 0x61, // payload
    ];

    publisher_stream.write(&publish_bytes_qos_1).unwrap();

    data = vec![0; 100];
    match publisher_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x40, 0x02, 0x00, 0x0A]);
        }
        _ => {
            panic!();
        }
    }

    data = vec![0; 100];
    match subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], publish_bytes_qos_1);
        }
        _ => {
            panic!();
        }
    }

    server.shutdown().unwrap();
}

#[test]
fn should_publish_message_with_both_qos_with_same_user() {
    let server = ServerTest::start("0.0.0.0:2538".to_string());

    let mut publisher_subscriber_stream = TcpStream::connect("0.0.0.0:2538".to_string()).unwrap();

    let mut data = vec![0; 100];

    let connect_bytes = [
        0x10, // packet type
        0x0E, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
        0x04, // protocol name
        0x00, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x63, 0x61, // client identifier
    ];

    publisher_subscriber_stream.write(&connect_bytes).unwrap();

    match publisher_subscriber_stream.read(&mut data) {
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

    publisher_subscriber_stream.write(&subscribe_bytes).unwrap();

    data = vec![0; 100];
    match publisher_subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x01]);
        }
        _ => {
            panic!();
        }
    }

    // PUBLICA con qos 0
    let publish_bytes_qos_0 = [
        0x30, // tiene la información del packet type 0011, dup flag + qos flag + retain flag
        0x0C, // remaining length
        0x00, 0x03, 0x61, 0x2F, 0x62, // topic name
        0x00, 0x0A, // packet identifier
        0x00, 0x03, 0x61, 0x2F, 0x61, // payload
    ];

    publisher_subscriber_stream
        .write(&publish_bytes_qos_0)
        .unwrap();

    data = vec![0; 100];
    match publisher_subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], publish_bytes_qos_0);
        }
        _ => {
            panic!();
        }
    }

    // PUBLICA con qos 1
    let publish_bytes_qos_1 = [
        0x32, // tiene la información del packet type 0011, dup flag + qos flag + retain flag
        0x0C, // remaining length
        0x00, 0x03, 0x61, 0x2F, 0x62, // topic name
        0x00, 0x0A, // packet identifier
        0x00, 0x03, 0x61, 0x2F, 0x61, // payload
    ];

    publisher_subscriber_stream
        .write(&publish_bytes_qos_1)
        .unwrap();

    data = vec![0; 100];
    match publisher_subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x40, 0x02, 0x00, 0x0A]);
        }
        _ => {
            panic!();
        }
    }

    data = vec![0; 100];
    match publisher_subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], publish_bytes_qos_1);
        }
        _ => {
            panic!();
        }
    }

    server.shutdown().unwrap();
}

#[test]
fn should_subscribe_qos0_and_publish_qos1() {
    let server = ServerTest::start("0.0.0.0:2539".to_string());

    let mut subscriber_stream = TcpStream::connect("0.0.0.0:2539".to_string()).unwrap();
    let mut publisher_stream = TcpStream::connect("0.0.0.0:2539".to_string()).unwrap();

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

    // PUBLICA con qos 1
    let publish_bytes_qos_1 = [
        0x32, // tiene la información del packet type 0011, dup flag + qos flag + retain flag
        0x0C, // remaining length
        0x00, 0x03, 0x61, 0x2F, 0x62, // topic name
        0x00, 0x0A, // packet identifier
        0x00, 0x03, 0x61, 0x2F, 0x61, // payload
    ];

    publisher_stream.write(&publish_bytes_qos_1).unwrap();

    data = vec![0; 100];
    match publisher_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x40, 0x02, 0x00, 0x0A]);
        }
        _ => {
            panic!();
        }
    }

    data = vec![0; 100];
    match subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(
                data[0..size],
                [0x30, 0x0A, 0x00, 0x03, 0x61, 0x2F, 0x62, 0x00, 0x03, 0x61, 0x2F, 0x61]
            );
        }
        _ => {
            panic!();
        }
    }

    server.shutdown().unwrap();
}

#[test]
fn should_publish_when_subscribe_has_invalid_topics() {
    let server = ServerTest::start("0.0.0.0:2541".to_string());

    let mut subscriber_stream = TcpStream::connect("0.0.0.0:2541".to_string()).unwrap();
    let mut publisher_stream = TcpStream::connect("0.0.0.0:2541".to_string()).unwrap();

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

    let subscribe_bytes = [
        0x82, // packet type
        0x0E, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
        0x00, 0x03, 0x61, 0x2F, 0x62, 0x00, // payload MQTT como mensaje + qos
        0x00, 0x03, 0x61, 0x2F, 0x64, 0x03, // payload MQTT como mensaje + qos
    ];

    subscriber_stream.write(&subscribe_bytes).unwrap();

    data = vec![0; 100];
    match subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x90, 0x04, 0x00, 0x0A, 0x00, 0x80]);
        }
        _ => {
            panic!();
        }
    }

    // PUBLICA con qos 1
    let publish_bytes_qos_1 = [
        0x32, // tiene la información del packet type 0010, dup flag + qos flag + retain flag
        0x0C, // remaining length
        0x00, 0x03, 0x61, 0x2F, 0x62, // topic name
        0x00, 0x0A, // packet identifier
        0x00, 0x03, 0x61, 0x2F, 0x61, // payload
    ];

    publisher_stream.write(&publish_bytes_qos_1).unwrap();

    data = vec![0; 100];
    match publisher_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x40, 0x02, 0x00, 0x0A]);
        }
        _ => {
            panic!();
        }
    }

    data = vec![0; 100];
    match subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(
                data[0..size],
                [0x30, 0x0A, 0x00, 0x03, 0x61, 0x2F, 0x62, 0x00, 0x03, 0x61, 0x2F, 0x61]
            );
        }
        _ => {
            panic!();
        }
    }

    // let puback_bytes = [
    //     0x40, //packet type puback
    //     0x02, //remainign length
    //     0x00, 0x0A, //packet identifier
    // ];
    // subscriber_stream.write(&puback_bytes).unwrap();

    let publish_bytes_qos_1_with_error = [
        0b00110010, // tiene la información del packet type 0011, dup flag + qos flag + retain flag
        0x0C,       // remaining length
        0x00, 0x03, 0x61, 0x2F, 0x64, // topic name
        0x00, 0x0A, // packet identifier
        0x00, 0x03, 0x61, 0x2F, 0x61, // payload
    ];

    publisher_stream
        .write(&publish_bytes_qos_1_with_error)
        .unwrap();

    data = vec![0; 100];
    match publisher_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [64, 2, 0, 10]);
        }
        _ => {
            panic!();
        }
    }

    sleep(Duration::from_secs(5));
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

#[test]
fn should_not_publish_if_client_is_not_connected() {
    let server = ServerTest::start("0.0.0.0:2540".to_string());

    let mut publisher_stream = TcpStream::connect("0.0.0.0:2540".to_string()).unwrap();

    let mut data = vec![0; 100];

    let publish_bytes = [
        0x30, // tiene la información del packet type 0011, dup flag + qos flag + retain flag
        0x0C, // remaining length
        0x00, 0x03, 0x61, 0x2F, 0x62, // topic name
        0x00, 0x0A, // packet identifier
        0x00, 0x03, 0x61, 0x2F, 0x61, // payload
    ];
    publisher_stream.write(&publish_bytes).unwrap();

    match publisher_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], []);
        }
        _ => {
            panic!();
        }
    }

    server.shutdown().unwrap();
}
