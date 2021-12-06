use crate::integration::setup::ServerTest;

use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::Mutex;

#[test]
fn should_unsubscribe_correctly() {
    let server = ServerTest::start("0.0.0.0:1889".to_string());
    let mut stream = TcpStream::connect("0.0.0.0:1889".to_string()).unwrap();

    let connect_bytes = [
        0x10, // packet type
        0x10, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
        0x04, // protocol name
        0x00, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x04, 0x62, 0x6F, 0x63, 0x61, // client identifier
    ];

    stream.write(&connect_bytes).unwrap();

    let mut data = vec![0; 1000];
    stream.read(&mut data).unwrap();

    let subscribe_bytes = [
        0x80, // packet type
        0x09, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x00, // payload MQTT como mensaje + qos
    ];
    stream.write(&subscribe_bytes).unwrap();
    
    stream.read(&mut data).unwrap();

    let unsubscribe_bytes = [
        0xA0, // packet type
        0x08, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // payload MQTT como mensaje
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
    let server = ServerTest::start("0.0.0.0:1892".to_string());
    let mut stream_subscriber = TcpStream::connect("0.0.0.0:1892".to_string()).unwrap();
    let mut stream_publisher = TcpStream::connect("0.0.0.0:1892".to_string()).unwrap();

    let mut data = vec![0; 1000];

    // CONNECT
    let connect_bytes_subscriber = [
        0x10, // packet type
        0x0E, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
        0x04, // protocol name
        0x00, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x62, 0x6F, // client identifier
    ];

    let connect_bytes_publisher = [
        0x10, // packet type
        0x0E, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
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
        0x80, // packet type
        0x09, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x00, // payload MQTT como mensaje
    ];

    stream_subscriber.write(&subscribe_bytes).unwrap();

    match stream_subscriber.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x00]);
        }
        _ => {
            panic!();
        }
    }

    // PUBLISH
    let publish_bytes = [
        0x32, // tiene la información del packet type 0011, dup flag + qos flag + retain flag
        0x0D, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // topic name
        0x00, 0x0A, // packet identifier
        0x00, 0x03, 0x61, 0x2F, 0x61, // payload
    ];

    stream_publisher.write(&publish_bytes).unwrap();

    match stream_subscriber.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x00, 0x03, 0x61, 0x2F, 0x61]);
        }
        _ => {
            panic!();
        }
    }

    //UNSUBSCRIBE
    // let unsubscribe_bytes = [
    //     0xA0, // packet type
    //     0x08, // remaining length
    //     0x00, 0x0A, // variable header, en particular packet identifier
    //     0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // payload MQTT como mensaje
    // ];
    // stream_subscriber.write(&unsubscribe_bytes).unwrap();

    // match stream_subscriber.read(&mut data) {
    //     Ok(size) => {
    //         assert_eq!(data[0..size], [0xB0, 0x02, 0x00, 0x0A]);
    //     }
    //     _ => {
    //         panic!();
    //     }
    // }


    stream_publisher.write(&publish_bytes).unwrap();

    let subscribe_bytes = [
        0x80, // packet type
        0x09, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x00, // payload MQT como topic
    ];

    stream_subscriber.write(&subscribe_bytes).unwrap();

    match stream_subscriber.read(&mut data) {
        Ok(size) => {
            println!("{:?}", &data[0..size]);
            assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x00]);
        }
        _ => {
            panic!();
        }
    }

    server.shutdown().unwrap();
}

#[test]
fn should_subscribe_some_topics_05() {
    let server = ServerTest::start("0.0.0.0:1892".to_string());
    let mut stream = TcpStream::connect("0.0.0.0:1892".to_string()).unwrap();

    let mut data = vec![0; 1000];

    let connect_bytes = [
        0x10, // packet type
        0x10, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
        0x04, // protocol name
        0x00, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x04, 0x62, 0x6F, 0x63, 0x61, // client identifier
    ];
    stream.write(&connect_bytes).unwrap();
    stream.read(&mut data).unwrap();

    let subscribe_bytes = [
        0x80, // packet type
        0x14, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x03, // payload MQTT como mensaje
        0x00, 0x03, 0x4D, 0x15, 0x45, 0x00, // payload MQT como mensaje
        0x00, 0x02, 0x4D, 0x15, 0x01, // payload MQ como mensaje
    ];

    stream.write(&subscribe_bytes).unwrap();

    match stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x80, 0x00, 0x01]);
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