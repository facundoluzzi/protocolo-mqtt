use crate::integration::setup::ServerTest;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

#[test]
fn testing_retained_message_new_subscribe_must_receive_the_retained_message() {
    let server = ServerTest::start("0.0.0.0:1953".to_string());

    let mut stream_to_create_topic = TcpStream::connect("0.0.0.0:1953".to_string()).unwrap();
    let mut stream_to_verify_retained_messages =
        TcpStream::connect("0.0.0.0:1953".to_string()).unwrap();
    let mut stream_to_publish_message = TcpStream::connect("0.0.0.0:1953".to_string()).unwrap();

    let subscribe_to_verify_retained_message = [
        0x10, // Packet Type
        0x0E, // Remaining Length
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, // MQTT
        0x04, // Protocol Name - SIEMPRE en 04 o falla
        0x00, // Flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x61, 0x60, // Client Identifier
    ];

    let publisher_alteg_connect_bytes = [
        0x10, // Packet Type
        0x0E, // Remaining Length
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, // MQTT
        0x04, // Protocol Name - SIEMPRE en 04 o falla
        0x00, // Flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x72, 0x61, // Client Identifier
    ];

    let mut data = vec![0; 100];

    stream_to_publish_message
        .write(&publisher_alteg_connect_bytes)
        .unwrap();

    match stream_to_publish_message.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
        }
        _ => {
            panic!();
        }
    }

    stream_to_verify_retained_messages
        .write(&subscribe_to_verify_retained_message)
        .unwrap();

    match stream_to_verify_retained_messages.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
        }
        _ => {
            panic!();
        }
    }

    let subscribe_bytes_crear_cantidad_de_goles = [
        0x82, // Packet Type
        0x20, // Remaining Length ---------> RECALCULAR PORQUE ESTA MAL
        0x00, 0x0A, // Variable Header, en particular packet identifier
        0x00, 0x1B, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
        0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, 0x64,
        0x65, 0x67, 0x6F, 0x6C, 0x65, 0x73, // TOPIC: Futbol/Boca/CantidadDeGoles
        0x01,
    ];

    data = vec![0; 100];
    stream_to_create_topic
        .write(&subscribe_bytes_crear_cantidad_de_goles)
        .unwrap();

    match stream_to_create_topic.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x01]);
        }
        _ => {
            panic!();
        }
    }

    let bytes_to_publish_message_alteg = [
        0x31, // Paquete publish
        0x26, // Remaining Length
        0x00, 0x1B, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
        0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, 0x64,
        0x65, 0x67, 0x6F, 0x6C, 0x65, 0x73, // // TOPIC: Futbol/Boca/CantidadDeGoles
        0x00, 0x11, // Packet Identifier
        0x00, 0x05, 0x41, 0x4C, 0x54, 0x45, 0x47, // Payload 'ALTEG'
    ];
    stream_to_publish_message
        .write(&bytes_to_publish_message_alteg)
        .unwrap();

    data = vec![0; 100];
    stream_to_verify_retained_messages
        .write(&subscribe_bytes_crear_cantidad_de_goles)
        .unwrap();

    match stream_to_verify_retained_messages.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x01]);
        }
        _ => {
            panic!();
        }
    }

    data = vec![0; 100];
    match stream_to_verify_retained_messages.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], bytes_to_publish_message_alteg);
        }
        _ => {
            panic!();
        }
    }

    server.shutdown().unwrap();
}

#[test]
fn testing_retained_message_one_publish_with_retain_and_another_without_retain_must_receive_the_first(
) {
    let server = ServerTest::start("0.0.0.0:2322".to_string());

    let mut stream_to_create_topic = TcpStream::connect("0.0.0.0:2322".to_string()).unwrap();
    let mut stream_to_verify_retained_messages =
        TcpStream::connect("0.0.0.0:2322".to_string()).unwrap();
    let mut stream_to_publish_message_alteg =
        TcpStream::connect("0.0.0.0:2322".to_string()).unwrap();
    let mut stream_to_publish_message_river =
        TcpStream::connect("0.0.0.0:2322".to_string()).unwrap();

    let subscribe_to_create_topic = [
        0x10, // Packet Type
        0x0E, // Remaining Length
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, // MQTT
        0x04, // Protocol Name - SIEMPRE en 04 o falla
        0x00, // Flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x62, 0x63, // Client Identifier
    ];

    let subscribe_to_verify_retained_message = [
        0x10, // Packet Type
        0x0E, // Remaining Length
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, // MQTT
        0x04, // Protocol Name - SIEMPRE en 04 o falla
        0x00, // Flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x61, 0x60, // Client Identifier
    ];

    let publisher_alteg_connect_bytes = [
        0x10, // Packet Type
        0x0E, // Remaining Length
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, // MQTT
        0x04, // Protocol Name - SIEMPRE en 04 o falla
        0x00, // Flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x72, 0x61, // Client Identifier
    ];

    let publisher_boca_connect_bytes = [
        0x10, // Packet Type
        0x0E, // Remaining Length
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, // MQTT
        0x04, // Protocol Name - SIEMPRE en 04 o falla
        0x00, // Flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x74, 0x55, // Client Identifier
    ];

    stream_to_create_topic
        .write(&subscribe_to_create_topic)
        .unwrap();

    let mut data = vec![0; 100];
    match stream_to_create_topic.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
        }
        _ => {
            panic!();
        }
    }

    stream_to_verify_retained_messages
        .write(&subscribe_to_verify_retained_message)
        .unwrap();

    let mut data = vec![0; 100];
    match stream_to_verify_retained_messages.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
        }
        _ => {
            panic!();
        }
    }

    stream_to_publish_message_alteg
        .write(&publisher_alteg_connect_bytes)
        .unwrap();
    data = vec![0; 100];
    match stream_to_publish_message_alteg.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
        }
        _ => {
            panic!();
        }
    }

    stream_to_publish_message_river
        .write(&publisher_boca_connect_bytes)
        .unwrap();
    data = vec![0; 100];
    match stream_to_publish_message_river.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
        }
        _ => {
            panic!();
        }
    }

    let subscribe_bytes_crear_cantidad_de_goles = [
        0x82, // Packet Type
        0x20, // Remaining Length
        0x00, 0x0A, // Variable Header, en particular packet identifier
        0x00, 0x1B, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
        0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, 0x64,
        0x65, 0x67, 0x6F, 0x6C, 0x65, 0x73, // TOPIC: Futbol/Boca/CantidadDeGoles
        0x01,
    ];

    data = vec![0; 100];
    stream_to_create_topic
        .write(&subscribe_bytes_crear_cantidad_de_goles)
        .unwrap();

    match stream_to_create_topic.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x01]);
        }
        _ => {
            panic!();
        }
    }

    let bytes_to_publish_message_alteg = [
        0x31, // Paquete publish
        0x26, // Remaining Length
        0x00, 0x1B, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
        0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, 0x64,
        0x65, 0x67, 0x6F, 0x6C, 0x65, 0x73, // // TOPIC: Futbol/Boca/CantidadDeGoles
        0x00, 0x11, // Packet Identifier
        0x00, 0x05, 0x41, 0x4C, 0x54, 0x45, 0x47, // Payload 'ALTEG'
    ];
    stream_to_publish_message_alteg
        .write(&bytes_to_publish_message_alteg)
        .unwrap();

    let bytes_to_publish_message_river = [
        0x32, // Paquete publish
        0x26, // Remaining Length
        0x00, 0x1B, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
        0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, 0x64,
        0x65, 0x67, 0x6F, 0x6C, 0x65, 0x73, // Topic: Futbol/Boca/CantidadDeGoles
        0x00, 0x11, // Packet Identifier
        0x00, 0x05, 0x72, 0x69, 0x76, 0x65, 0x72, // Payload 'River'
    ];

    stream_to_publish_message_river
        .write(&bytes_to_publish_message_river)
        .unwrap();
    data = vec![0; 100];
    match stream_to_publish_message_river.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x40, 0x02, 0x00, 0x11]);
        }
        _ => {
            panic!();
        }
    }

    stream_to_verify_retained_messages
        .write(&subscribe_bytes_crear_cantidad_de_goles)
        .unwrap();

    data = vec![0; 5];
    match stream_to_verify_retained_messages.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x01]);
        }
        _ => {
            panic!();
        }
    }

    data = vec![0; 100];
    match stream_to_verify_retained_messages.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], bytes_to_publish_message_alteg);
        }
        _ => {
            panic!();
        }
    }

    server.shutdown().unwrap();
}

#[test]
fn testing_retained_message_two_publish_with_retain_must_receive_the_last() {
    let server = ServerTest::start("0.0.0.0:2528".to_string());

    let mut stream_to_create_topic = TcpStream::connect("0.0.0.0:2528".to_string()).unwrap();
    let mut stream_to_verify_retained_messages =
        TcpStream::connect("0.0.0.0:2528".to_string()).unwrap();
    let mut stream_to_publish_message_alteg =
        TcpStream::connect("0.0.0.0:2528".to_string()).unwrap();
    let mut stream_to_publish_message_river =
        TcpStream::connect("0.0.0.0:2528".to_string()).unwrap();

    let subscribe_to_create_topic = [
        0x10, // Packet Type
        0x0E, // Remaining Length
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, // MQTT
        0x04, // Protocol Name - SIEMPRE en 04 o falla
        0x00, // Flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x62, 0x63, // Client Identifier
    ];

    let subscribe_to_verify_retained_message = [
        0x10, // Packet Type
        0x0E, // Remaining Length
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, // MQTT
        0x04, // Protocol Name - SIEMPRE en 04 o falla
        0x00, // Flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x61, 0x60, // Client Identifier
    ];

    let publisher_alteg_connect_bytes = [
        0x10, // Packet Type
        0x0E, // Remaining Length
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, // MQTT
        0x04, // Protocol Name - SIEMPRE en 04 o falla
        0x00, // Flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x72, 0x61, // Client Identifier
    ];

    let publisher_boca_connect_bytes = [
        0x10, // Packet Type
        0x0E, // Remaining Length
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, // MQTT
        0x04, // Protocol Name - SIEMPRE en 04 o falla
        0x00, // Flags
        0x00, 0x0B, // keep alive
        0x00, 0x02, 0x74, 0x55, // Client Identifier
    ];

    stream_to_create_topic
        .write(&subscribe_to_create_topic)
        .unwrap();

    let mut data = vec![0; 100];
    match stream_to_create_topic.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
        }
        _ => {
            panic!();
        }
    }

    stream_to_verify_retained_messages
        .write(&subscribe_to_verify_retained_message)
        .unwrap();

    let mut data = vec![0; 100];
    match stream_to_verify_retained_messages.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
        }
        _ => {
            panic!();
        }
    }

    stream_to_publish_message_alteg
        .write(&publisher_alteg_connect_bytes)
        .unwrap();
    data = vec![0; 100];
    match stream_to_publish_message_alteg.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
        }
        _ => {
            panic!();
        }
    }

    stream_to_publish_message_river
        .write(&publisher_boca_connect_bytes)
        .unwrap();
    data = vec![0; 100];
    match stream_to_publish_message_river.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
        }
        _ => {
            panic!();
        }
    }

    let subscribe_bytes_crear_cantidad_de_goles = [
        0x82, // Packet Type
        0x20, // Remaining Length
        0x00, 0x0A, // Variable Header, en particular packet identifier
        0x00, 0x1B, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
        0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, 0x64,
        0x65, 0x67, 0x6F, 0x6C, 0x65, 0x73, // TOPIC: Futbol/Boca/CantidadDeGoles
        0x01,
    ];

    stream_to_create_topic
        .write(&subscribe_bytes_crear_cantidad_de_goles)
        .unwrap();

    data = vec![0; 100];
    match stream_to_create_topic.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x01]);
        }
        _ => {
            panic!();
        }
    }

    let bytes_to_publish_message_alteg = [
        0x31, // Paquete publish
        0x26, // Remaining Length
        0x00, 0x1B, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
        0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, 0x64,
        0x65, 0x67, 0x6F, 0x6C, 0x65, 0x73, // // TOPIC: Futbol/Boca/CantidadDeGoles
        0x00, 0x11, // Packet Identifier
        0x00, 0x05, 0x41, 0x4C, 0x54, 0x45, 0x47, // Payload 'ALTEG'
    ];
    stream_to_publish_message_alteg
        .write(&bytes_to_publish_message_alteg)
        .unwrap();

    thread::sleep(Duration::from_millis(100));

    let bytes_to_publish_message_river = [
        0x31, // Paquete publish
        0x26, // Remaining Length
        0x00, 0x1B, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
        0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, 0x64,
        0x65, 0x67, 0x6F, 0x6C, 0x65, 0x73, // Topic: Futbol/Boca/CantidadDeGoles
        0x00, 0x11, // Packet Identifier
        0x00, 0x05, 0x72, 0x69, 0x76, 0x65, 0x72, // Payload 'River'
    ];

    stream_to_publish_message_river
        .write(&bytes_to_publish_message_river)
        .unwrap();

    thread::sleep(Duration::from_millis(100));

    stream_to_verify_retained_messages
        .write(&subscribe_bytes_crear_cantidad_de_goles)
        .unwrap();

    data = vec![0; 5];
    match stream_to_verify_retained_messages.read(&mut data) {
        Ok(size) => {
            assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x01]);
        }
        _ => {
            panic!();
        }
    }
    thread::sleep(Duration::from_millis(100));
    data = vec![0; 100];
    match stream_to_verify_retained_messages.read(&mut data) {
        Ok(size) => {
            let expected_bytes = [
                0x31, // Paquete publish
                0x26, // Remaining Length
                0x00, 0x1B, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
                0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64,
                0x64, 0x65, 0x67, 0x6F, 0x6C, 0x65,
                0x73, // Topic: Futbol/Boca/CantidadDeGoles
                0x00, 0x11, // Packet Identifier
                0x00, 0x05, 0x72, 0x69, 0x76, 0x65, 0x72, // Payload 'River'
            ];
            assert_eq!(data[0..size], expected_bytes);
        }
        _ => {
            panic!();
        }
    }
    server.shutdown().unwrap();
}
