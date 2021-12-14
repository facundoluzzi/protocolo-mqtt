use crate::integration::setup::ServerTest;
use core::time;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

#[test]
fn should_receive_second_publish_with_dup() {
    let server = ServerTest::start("0.0.0.0:4545".to_string());

    let mut subscriber_stream = TcpStream::connect("0.0.0.0:4545".to_string()).unwrap();
    let mut publisher_stream = TcpStream::connect("0.0.0.0:4545".to_string()).unwrap();

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
    std::thread::sleep(time::Duration::from_secs(6));

    data = vec![0; 100];
    match subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(
                data[0..size],
                [
                    0b00111000, // tiene la información del packet type 0011, dup flag + qos flag + retain flag
                    0x0C,       // remaining length
                    0x00, 0x03, 0x61, 0x2F, 0x62, // topic name
                    0x00, 0x0A, // packet identifier
                    0x00, 0x03, 0x61, 0x2F, 0x61, // payload
                ]
            );
        }
        _ => {
            panic!();
        }
    }

    server.shutdown().unwrap();
}
#[test]
fn should_receive_third_publish_with_dup() {
    let server = ServerTest::start("0.0.0.0:4301".to_string());

    let mut subscriber_stream = TcpStream::connect("0.0.0.0:4301".to_string()).unwrap();
    let mut publisher_stream = TcpStream::connect("0.0.0.0:4301".to_string()).unwrap();

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
    std::thread::sleep(time::Duration::from_secs(6));

    data = vec![0; 100];
    match subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(
                data[0..size],
                [
                    0b00111000, // tiene la información del packet type 0011, dup flag + qos flag + retain flag
                    0x0C,       // remaining length
                    0x00, 0x03, 0x61, 0x2F, 0x62, // topic name
                    0x00, 0x0A, // packet identifier
                    0x00, 0x03, 0x61, 0x2F, 0x61, // payload
                ]
            );
        }
        _ => {
            panic!();
        }
    }
    std::thread::sleep(time::Duration::from_secs(6));

    data = vec![0; 100];
    match subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(
                data[0..size],
                [
                    0b00111000, // tiene la información del packet type 0011, dup flag + qos flag + retain flag
                    0x0C,       // remaining length
                    0x00, 0x03, 0x61, 0x2F, 0x62, // topic name
                    0x00, 0x0A, // packet identifier
                    0x00, 0x03, 0x61, 0x2F, 0x61, // payload
                ]
            );
        }
        _ => {
            panic!();
        }
    }

    server.shutdown().unwrap();
}



#[test]
fn should_receive_second_publish_with_dup_and_after_puback_not_receive_more() {
    let server = ServerTest::start("0.0.0.0:2533".to_string());

    let mut subscriber_stream = TcpStream::connect("0.0.0.0:2533".to_string()).unwrap();
    let mut publisher_stream = TcpStream::connect("0.0.0.0:2533".to_string()).unwrap();

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
    std::thread::sleep(time::Duration::from_secs(6));

    data = vec![0; 100];
    match subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(
                data[0..size],
                [
                    0b00111000, // tiene la información del packet type 0011, dup flag + qos flag + retain flag
                    0x0C,       // remaining length
                    0x00, 0x03, 0x61, 0x2F, 0x62, // topic name
                    0x00, 0x0A, // packet identifier
                    0x00, 0x03, 0x61, 0x2F, 0x61, // payload
                ]
            );
        }
        _ => {
            panic!();
        }
    }
    let puback_bytes = [
        0x40, //packet type puback
        0x02,   //remainign length
        0x00, 0x0A  //packet idezntifier
    ];


    subscriber_stream.write(&puback_bytes).unwrap();
    std::thread::sleep(time::Duration::from_secs(10));

    data = vec![0; 100];
    match subscriber_stream.read(&mut data) {
        Ok(size) => {
            assert_eq!(
                data[0..size],
                []
            );
        }
        _ => {
            panic!();
        }
    }

    server.shutdown().unwrap();
}