use crate::integration::setup::ServerTest;

use core::time;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::thread;

#[test]
fn should_unsubscribe_correctly() {
    let server = ServerTest::start("0.0.0.0:2010".to_string());
    let mut stream = TcpStream::connect("0.0.0.0:2010".to_string()).unwrap();

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
        0x08, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, // payload MQTT como mensaje
    ];
    stream.write(&unsubscribe_bytes).unwrap();

    match stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0xB0, 0x02, 0x00, 0x0A]);
        }
        _ => {
            panic!();
        }
    }

    server.shutdown().unwrap();
}

#[test]
fn should_subscribe_receive_publish_and_unsubscribe() {
    let server = ServerTest::start("0.0.0.0:2011".to_string());
    let mut stream_subscriber = TcpStream::connect("0.0.0.0:2011".to_string()).unwrap();
    let mut stream_publisher = TcpStream::connect("0.0.0.0:2011".to_string()).unwrap();

    let mut data = vec![0; 100];

    // CONNECT
    let connect_bytes_subscriber = [
        0x10, // packet type
        0x0E, // remaining length
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, //mqtt
        0x04, // protocol name
        0x00, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x62, 0x6F, // client identifier
    ];

    let connect_bytes_publisher = [
        0x10, // packet type
        0x0E, // remaining length
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, //mqtt
        0x04, // protocol name
        0x00, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x63, 0x61, // client identifier
    ];

    stream_subscriber.write(&connect_bytes_subscriber).unwrap();
    stream_subscriber.read(&mut data).unwrap();

    stream_publisher.write(&connect_bytes_publisher).unwrap();
    stream_publisher.read(&mut data).unwrap();

    // SUBSCRIBE
    let subscribe_bytes = [
        0x82, // packet type
        0x09, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, 0x01, // payload MQTT como mensaje
    ];

    stream_subscriber.write(&subscribe_bytes).unwrap();

    data = vec![0; 100];
    match stream_subscriber.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x01]);
        }
        _ => {
            panic!();
        }
    }

    // PUBLISH
    let publish_bytes = [
        0x32, // tiene la información del packet type 0011, dup flag + qos flag + retain flag
        0x0D, // remaining length
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, // topic name
        0x00, 0x0A, // packet identifier
        0x00, 0x03, 0x61, 0x2F, 0x62, // payload
    ];

    stream_publisher.write(&publish_bytes).unwrap();

    let puback_bytes = [
        0x40, //packet type puback
        0x02, //remainign length
        0x00, 0x0A, //packet identifier
    ];

    thread::sleep(time::Duration::from_millis(100));

    data = vec![0; 100];
    match stream_publisher.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], puback_bytes);
        }
        _ => {
            panic!();
        }
    }

    data = vec![0; 100];
    match stream_subscriber.read(&mut data) {
        Ok(size) => {
            let expected_packet = [
                0x32, 0x0D, 0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, 0x00, 0x0A, 0x00, 0x03, 0x61, 0x2F,
                0x62,
            ];
            assert_eq!(data[0..size], expected_packet);
        }
        _ => {
            panic!();
        }
    }

    stream_subscriber.write(&puback_bytes).unwrap();

    //UNSUBSCRIBE
    let unsubscribe_bytes = [
        0xA2, // packet type
        0x08, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, // payload MQTT como mensaje
    ];

    thread::sleep(time::Duration::from_millis(100));
    stream_subscriber.write(&unsubscribe_bytes).unwrap();

    data = vec![0; 100];
    match stream_subscriber.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0xB0, 0x02, 0x00, 0x0A]);
        }
        _ => {
            panic!();
        }
    }

    stream_publisher.write(&publish_bytes).unwrap();
    data = vec![0; 100];

    match stream_subscriber.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], []);
        }
        _ => {
            panic!();
        }
    }

    server.shutdown().unwrap();
}
