use crate::integration::setup::ServerTest;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn should_connect_and_cant_subscribe_with_keep_alive() {
    let server = ServerTest::start("0.0.0.0:1897".to_string());
    let mut stream = TcpStream::connect("0.0.0.0:1897".to_string()).unwrap();

    let connect_bytes = [
        0x10, // packet type
        0x10, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
        0x04, // protocol name
        0x00, // flags
        0x00, 0x02, // keep alive
        0x00, 0x04, 0x62, 0x6F, 0x63, 0x61, // client identifier
    ];

    stream.write(&connect_bytes).unwrap();

    let mut data = vec![0; 100];
    let size = stream.read(&mut data).unwrap();

    assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);

    let subscribe_bytes = [
        0x80, // packet type
        0x09, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x00, // payload MQTT como mensaje + qos
    ];

    sleep(Duration::from_secs(2));

    stream.write(&subscribe_bytes).unwrap();

    data = vec![0; 100];
    let size = stream.read(&mut data).unwrap();
    assert_eq!(data[0..size], [144, 0x03, 0, 0x0A, 0]);

    sleep(Duration::from_secs(4));

    let subscribe_bytes_2 = [
        0x80, // packet type
        0x09, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x00, // payload MQTT como mensaje + qos
    ];
    stream.write(&subscribe_bytes_2).unwrap();

    data = vec![0; 100];
    let size = stream.read(&mut data).unwrap();
    assert_eq!(data[0..size], []);

    server.shutdown().unwrap();
}

#[test]
fn should_connect_and_can_subscribe_with_null_keep_alive() {
    let server = ServerTest::start("0.0.0.0:1897".to_string());
    let mut stream = TcpStream::connect("0.0.0.0:1897".to_string()).unwrap();

    let connect_bytes = [
        0x10, // packet type
        0x10, // remaining length
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
        0x04, // protocol name
        0x00, // flags
        0x00, 0x00, // keep alive
        0x00, 0x04, 0x62, 0x6F, 0x63, 0x61, // client identifier
    ];

    stream.write(&connect_bytes).unwrap();

    let mut data = vec![0; 100];
    let size = stream.read(&mut data).unwrap();

    assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);

    let subscribe_bytes = [
        0x80, // packet type
        0x09, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, 0x00, // payload MQTT como mensaje + qos
    ];

    stream.write(&subscribe_bytes).unwrap();

    data = vec![0; 100];
    let size = stream.read(&mut data).unwrap();
    assert_eq!(data[0..size], [144, 0x03, 0, 0x0A, 0]);

    sleep(Duration::from_secs(4));

    let subscribe_bytes_2 = [
        0x80, // packet type
        0x09, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
        0x00, 0x04, 0x4D, 0x15, 0x45, 0x15, 0x00, // payload MQTT como mensaje + qos
    ];
    stream.write(&subscribe_bytes_2).unwrap();

    data = vec![0; 100];
    let size = stream.read(&mut data).unwrap();
    assert_eq!(data[0..size], [144, 0x03, 0, 0x0A, 0]);

    server.shutdown().unwrap();
}
