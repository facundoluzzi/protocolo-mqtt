use crate::integration::setup::ServerTest;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

#[test]
fn connect_with_invalid_mqtt_packet_should_disconnect() {
    let server = ServerTest::start("0.0.0.0:3477".to_string());
    let mut stream = TcpStream::connect("0.0.0.0:3477".to_string()).unwrap();

    let connect_bytes = [
        0x10, // packet type
        0x10, // remaining length
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x51, //mqtt
        0x04, // protocol name
        0x00, // flags
        0x00, 0x0B, // keep alive
        0x00, 0x04, 0x62, 0x6F, 0x63, 0x61, // client identifier
    ];

    stream.write(&connect_bytes).unwrap();

    let mut data = vec![0; 100];

    let subscribe_bytes = [
        0x80, // packet type
        0x09, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, 0x00, // payload MQTT como mensaje + qos
    ];

    stream.write(&subscribe_bytes).unwrap();

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
