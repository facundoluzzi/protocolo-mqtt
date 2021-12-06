mod tests {
    use crate::integration::setup::ServerTest;
    use std::{
        io::{Read, Write},
        net::TcpStream,
    };

    #[test]
    fn testing_subscribe_and_publish_with_wildcard_astherisc_after_the_bar() {
        let server = ServerTest::start("0.0.0.0:1893".to_string());

        let mut stream_to_create_topic_goles =
            TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();
        let mut stream_to_create_topic_partidos =
            TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();
        let mut stream_to_subscribe_with_wildcard =
            TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();
        let mut stream_to_publish_message = TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();

        let subscriber_goles_connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x62, 0x63, // Client Identifier
        ];

        let subscriber_partidos_connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x00, 0x03, // Client Identifier
        ];

        let subscriber_with_wildcard_connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x00, 0x01, // Client Identifier
        ];

        let publisher_connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x00, 0x02, // Client Identifier
        ];

        stream_to_create_topic_goles
            .write(&subscriber_goles_connect_bytes)
            .unwrap();
        stream_to_create_topic_partidos
            .write(&subscriber_partidos_connect_bytes)
            .unwrap();
        stream_to_subscribe_with_wildcard
            .write(&subscriber_with_wildcard_connect_bytes)
            .unwrap();
        stream_to_publish_message
            .write(&publisher_connect_bytes)
            .unwrap();

        let mut data = vec![0; 100];
        match stream_to_create_topic_goles.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }
        data = vec![0; 100];
        match stream_to_subscribe_with_wildcard.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }
        data = vec![0; 100];
        match stream_to_publish_message.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }
        data = vec![0; 100];
        match stream_to_create_topic_partidos.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }

        let subscribe_bytes_crear_cantidad_de_goles = [
            0x80, // Packet Type
            0x20, // Remaining Length ---------> RECALCULAR PORQUE ESTA MAL
            0x00, 0x0A, // Variable Header, en particular packet identifier
            0x00, 0x1B, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
            0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64,
            0x64, 0x65, 0x67, 0x6F, 0x6C, 0x65, 0x73, // TOPIC: Futbol/Boca/CantidadDeGoles
            0x00,
        ];
        data = vec![0; 100];
        stream_to_create_topic_goles
            .write(&subscribe_bytes_crear_cantidad_de_goles)
            .unwrap();

        match stream_to_create_topic_goles.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x00]);
            }
            _ => {
                panic!();
            }
        }

        let subscribe_bytes_crear_cantidad_de_partidos = [
            0x80, // Packet Type
            0x23, // Remaining Length
            0x00, 0x0A, // Variable Header, en particular packet identifier
            0x00, 0x1E, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, 0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F,
            0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, 0x64, 0x65, 0x70, 0x61, 0x72, 0x74,
            0x69, 0x64, 0x6F, 0x73, // TOPIC: Futbol/Boca/CantidadDePartidos
            0x00,
        ];
        stream_to_create_topic_partidos
            .write(&subscribe_bytes_crear_cantidad_de_partidos)
            .unwrap();
        match stream_to_create_topic_partidos.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x00]);
            }
            _ => {
                panic!();
            }
        }

        let subscribe_bytes_with_wildcard = [
            0x80, // Packet Type
            0x12, // Remaining Length
            0x00, 0x0A, // Variable Header, en particular packet identifier
            0x00, 0x0D, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
            0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x2A, // TOPIC: Futbol/Boca/*
            0x00,
        ];

        data = vec![0; 100];
        stream_to_subscribe_with_wildcard
            .write(&subscribe_bytes_with_wildcard)
            .unwrap();
        match stream_to_subscribe_with_wildcard.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x00]);
            }
            _ => {
                panic!();
            }
        }
        let bytes_to_publish = [
            0x32, // Paquete publish
            0x26, // Remaining Length
            0x00, 0x1B, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
            0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64,
            0x64, 0x65, 0x67, 0x6F, 0x6C, 0x65, 0x73, // // TOPIC: Futbol/Boca/CantidadDeGoles
            0x00, 0x11, // Packet Identifier
            0x00, 0x05, 0x41, 0x4C, 0x54, 0x45, 0x47, // Payload 'ALTEG'
        ];
        stream_to_publish_message.write(&bytes_to_publish).unwrap();
        data = vec![0; 100];
        match stream_to_publish_message.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x40, 0x01, 0x00, 0x11]);
            }
            _ => {
                panic!();
            }
        }
        data = vec![0; 100];
        match stream_to_subscribe_with_wildcard.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], bytes_to_publish);
            }
            _ => {
                panic!();
            }
        }
        server.shutdown().unwrap();
    }

    #[test]
    fn testing_subscribe_and_publish_with_wildcard_astherisc_at_the_end_of_word() {
        let server = ServerTest::start("0.0.0.0:1893".to_string());

        let mut stream_to_create_topic_goles =
            TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();
        let mut stream_to_create_topic_partidos =
            TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();
        let mut stream_to_subscribe_with_wildcard =
            TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();
        let mut stream_to_publish_message = TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();

        let subscriber_goles_connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x62, 0x63, // Client Identifier
        ];

        let subscriber_partidos_connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x00, 0x03, // Client Identifier
        ];

        let subscriber_with_wildcard_connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x00, 0x01, // Client Identifier
        ];

        let publisher_connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x00, 0x02, // Client Identifier
        ];

        stream_to_create_topic_goles
            .write(&subscriber_goles_connect_bytes)
            .unwrap();
        stream_to_create_topic_partidos
            .write(&subscriber_partidos_connect_bytes)
            .unwrap();
        stream_to_subscribe_with_wildcard
            .write(&subscriber_with_wildcard_connect_bytes)
            .unwrap();
        stream_to_publish_message
            .write(&publisher_connect_bytes)
            .unwrap();

        let mut data = vec![0; 100];
        match stream_to_create_topic_goles.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }
        data = vec![0; 100];
        match stream_to_subscribe_with_wildcard.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }
        data = vec![0; 100];
        match stream_to_publish_message.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }
        data = vec![0; 100];
        match stream_to_create_topic_partidos.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }

        let subscribe_bytes_crear_cantidad_de_goles = [
            0x80, // Packet Type
            0x20, // Remaining Length ---------> RECALCULAR PORQUE ESTA MAL
            0x00, 0x0A, // Variable Header, en particular packet identifier
            0x00, 0x1B, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
            0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64,
            0x64, 0x65, 0x67, 0x6F, 0x6C, 0x65, 0x73, // // TOPIC: Futbol/Boca/CantidadDeGoles
            0x00,
        ];
        data = vec![0; 100];
        stream_to_create_topic_goles
            .write(&subscribe_bytes_crear_cantidad_de_goles)
            .unwrap();

        match stream_to_create_topic_goles.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x00]);
            }
            _ => {
                panic!();
            }
        }

        let subscribe_bytes_crear_cantidad_de_partidos = [
            0x80, // Packet Type
            0x23, // Remaining Length
            0x00, 0x0A, // Variable Header, en particular packet identifier
            0x00, 0x1E, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
            0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64,
            0x64, 0x65, 0x70, 0x61, 0x72, 0x74, 0x69, 0x64, 0x6F,
            0x73, // TOPIC: Futbol/Boca/CantidadDePartidos
            0x00,
        ];
        stream_to_create_topic_partidos
            .write(&subscribe_bytes_crear_cantidad_de_partidos)
            .unwrap();
        match stream_to_create_topic_partidos.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x00]);
            }
            _ => {
                panic!();
            }
        }

        let subscribe_bytes_with_wildcard = [
            0x80, // Packet Type
            0x1C, // Remaining Length
            0x00, 0x0A, // Variable Header, en particular packet identifier
            0x00, 0x17, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Inicio Payload
            0x2F, 0x62, 0x6F, 0x63, 0x61, // boca
            0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, // cantidad
            0x64, 0x65, 0x2A, // TOPIC: Futbol/Boca/CantidadDe*
            0x00,
        ];

        data = vec![0; 100];
        stream_to_subscribe_with_wildcard
            .write(&subscribe_bytes_with_wildcard)
            .unwrap();
        match stream_to_subscribe_with_wildcard.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x00]);
            }
            _ => {
                panic!();
            }
        }
        let bytes_to_publish = [
            0x32, // Paquete publish
            0x26, // Remaining Length
            0x00, 0x1B, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
            0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64,
            0x64, 0x65, 0x67, 0x6F, 0x6C, 0x65, 0x73, // TOPIC: Futbol/Boca/CantidadDeGoles
            0x00, 0x11, // Packet Identifier
            0x00, 0x05, 0x41, 0x4C, 0x54, 0x45, 0x47, // Payload 'ALTEG'
        ];
        stream_to_publish_message.write(&bytes_to_publish).unwrap();
        data = vec![0; 100];
        match stream_to_publish_message.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x40, 0x01, 0x00, 0x11]);
            }
            _ => {
                panic!();
            }
        }
        data = vec![0; 100];
        match stream_to_subscribe_with_wildcard.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], bytes_to_publish);
            }
            _ => {
                panic!();
            }
        }
        server.shutdown().unwrap();
    }

    #[test]
    fn testing_subscribe_and_publish_with_wildcard_greater_than() {
        let server = ServerTest::start("0.0.0.0:1893".to_string());

        let mut stream_to_create_topic_goles =
            TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();
        let mut stream_to_create_topic_partidos =
            TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();
        let mut stream_to_subscribe_with_wildcard =
            TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();
        let mut stream_to_publish_message = TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();

        let subscriber_goles_connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x62, 0x63, // Client Identifier
        ];

        let subscriber_partidos_connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x00, 0x03, // Client Identifier
        ];

        let subscriber_with_wildcard_connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x00, 0x01, // Client Identifier
        ];

        let publisher_connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x00, 0x02, // Client Identifier
        ];

        stream_to_create_topic_goles
            .write(&subscriber_goles_connect_bytes)
            .unwrap();
        stream_to_create_topic_partidos
            .write(&subscriber_partidos_connect_bytes)
            .unwrap();
        stream_to_subscribe_with_wildcard
            .write(&subscriber_with_wildcard_connect_bytes)
            .unwrap();
        stream_to_publish_message
            .write(&publisher_connect_bytes)
            .unwrap();

        let mut data = vec![0; 100];
        match stream_to_create_topic_goles.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }
        data = vec![0; 100];
        match stream_to_subscribe_with_wildcard.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }
        data = vec![0; 100];
        match stream_to_publish_message.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }
        data = vec![0; 100];
        match stream_to_create_topic_partidos.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }

        let subscribe_bytes_crear_cantidad_de_goles = [
            0x80, // Packet Type
            0x25, // Remaining Length
            0x00, 0x0A, // Variable Header, en particular packet identifier
            0x00, 0x20, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
            0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64,
            0x64, 0x65, 0x67, 0x6F, 0x6C, 0x65, 0x73, 0x2F, 0x32, 0x30, 0x32,
            0x31, // TOPIC: Futbol/Boca/CantidadeDeGoles/2021
            0x00,
        ];
        data = vec![0; 100];
        stream_to_create_topic_goles
            .write(&subscribe_bytes_crear_cantidad_de_goles)
            .unwrap();

        match stream_to_create_topic_goles.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x00]);
            }
            _ => {
                panic!();
            }
        }

        let subscribe_bytes_crear_cantidad_de_partidos = [
            0x80, // Packet Type
            0x23, // Remaining Length
            0x00, 0x0A, // Variable Header, en particular packet identifier
            0x00, 0x1E, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, 0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F,
            0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, 0x64, 0x65, 0x70, 0x61, 0x72, 0x74,
            0x69, 0x64, 0x6F, 0x73, // TOPIC: Futbol/Boca/CantidadDePartidos
            0x00,
        ];
        stream_to_create_topic_partidos
            .write(&subscribe_bytes_crear_cantidad_de_partidos)
            .unwrap();
        match stream_to_create_topic_partidos.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x00]);
            }
            _ => {
                panic!();
            }
        }

        let subscribe_bytes_with_wildcard = [
            0x80, // Packet Type
            0x12, // Remaining Length
            0x00, 0x0A, // Variable Header, en particular packet identifier
            0x00, 0x0D, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
            0x2F, 0x62, 0x6F, 0x63, 0x61, // boca
            0x2F, 0x3E, // Topic :: futbol/boca/>
            0x00,
        ];

        data = vec![0; 100];
        stream_to_subscribe_with_wildcard
            .write(&subscribe_bytes_with_wildcard)
            .unwrap();
        match stream_to_subscribe_with_wildcard.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x00]);
            }
            _ => {
                panic!();
            }
        }
        let bytes_to_publish = [
            0x32, // Paquete publish
            0x2B, // Remaining Length
            0x00, 0x20, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
            0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64,
            0x64, 0x65, 0x67, 0x6F, 0x6C, 0x65, 0x73, 0x2F, 0x32, 0x30, 0x32,
            0x31, // Futbol/Boca/CantidadDeGoles/2021
            0x00, 0x11, // Packet Identifier
            0x00, 0x05, 0x41, 0x4C, 0x54, 0x45, 0x47, // Payload 'ALTEG'
        ];
        stream_to_publish_message.write(&bytes_to_publish).unwrap();
        data = vec![0; 100];
        match stream_to_publish_message.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x40, 0x01, 0x00, 0x11]);
            }
            _ => {
                panic!();
            }
        }
        data = vec![0; 100];
        match stream_to_subscribe_with_wildcard.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], bytes_to_publish);
            }
            _ => {
                panic!();
            }
        }
        server.shutdown().unwrap();
    }

    #[test]
    fn testing_subscribe_and_publish_with_wildcard_astherisc_and_greater_than() {
        let server = ServerTest::start("0.0.0.0:1893".to_string());

        let mut stream_to_create_topic_goles =
            TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();
        let mut stream_to_create_topic_partidos =
            TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();
        let mut stream_to_subscribe_with_wildcard =
            TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();
        let mut stream_to_publish_message = TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();

        let subscriber_goles_connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x62, 0x63, // Client Identifier
        ];

        let subscriber_partidos_connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x00, 0x03, // Client Identifier
        ];

        let subscriber_with_wildcard_connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x00, 0x01, // Client Identifier
        ];

        let publisher_connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x00, 0x02, // Client Identifier
        ];

        stream_to_create_topic_goles
            .write(&subscriber_goles_connect_bytes)
            .unwrap();
        stream_to_create_topic_partidos
            .write(&subscriber_partidos_connect_bytes)
            .unwrap();
        stream_to_subscribe_with_wildcard
            .write(&subscriber_with_wildcard_connect_bytes)
            .unwrap();
        stream_to_publish_message
            .write(&publisher_connect_bytes)
            .unwrap();

        let mut data = vec![0; 100];
        match stream_to_create_topic_goles.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }
        data = vec![0; 100];
        match stream_to_subscribe_with_wildcard.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }
        data = vec![0; 100];
        match stream_to_publish_message.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }
        data = vec![0; 100];
        match stream_to_create_topic_partidos.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }

        let subscribe_bytes_crear_cantidad_de_goles = [
            0x80, // Packet Type
            0x25, // Remaining Length
            0x00, 0x0A, // Variable Header, en particular packet identifier
            0x00, 0x20, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
            0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64,
            0x64, 0x65, 0x67, 0x6F, 0x6C, 0x65, 0x73, 0x2F, 0x32, 0x30, 0x32,
            0x31, // TOPIC: Futbol/Boca/CantidadeDeGoles/2021
            0x00,
        ];
        data = vec![0; 100];
        stream_to_create_topic_goles
            .write(&subscribe_bytes_crear_cantidad_de_goles)
            .unwrap();

        match stream_to_create_topic_goles.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x00]);
            }
            _ => {
                panic!();
            }
        }

        let subscribe_bytes_crear_cantidad_de_partidos = [
            0x80, // Packet Type
            0x23, // Remaining Length
            0x00, 0x0A, // Variable Header, en particular packet identifier
            0x00, 0x1E, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, 0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F,
            0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, 0x64, 0x65, 0x70, 0x61, 0x72, 0x74,
            0x69, 0x64, 0x6F, 0x73, // TOPIC: Futbol/Boca/CantidadDePartidos
            0x00,
        ];
        stream_to_create_topic_partidos
            .write(&subscribe_bytes_crear_cantidad_de_partidos)
            .unwrap();
        match stream_to_create_topic_partidos.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x00]);
            }
            _ => {
                panic!();
            }
        }

        let subscribe_bytes_with_wildcard = [
            0x80, // Packet Type
            0x0D, // Remaining Length
            0x00, 0x0A, // Variable Header, en particular packet identifier
            0x00, 0x08, 0x2A, // Inicio Payload
            0x2F, 0x62, 0x6F, 0x63, 0x61, // boca
            0x2F, 0x3E, // Topic :: futbol/boca/>
            0x00,
        ];

        data = vec![0; 100];
        stream_to_subscribe_with_wildcard
            .write(&subscribe_bytes_with_wildcard)
            .unwrap();
        match stream_to_subscribe_with_wildcard.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x00]);
            }
            _ => {
                panic!();
            }
        }
        let bytes_to_publish = [
            0x32, // Paquete publish
            0x2B, // Remaining Length
            0x00, 0x20, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
            0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64,
            0x64, 0x65, 0x67, 0x6F, 0x6C, 0x65, 0x73, 0x2F, 0x32, 0x30, 0x32,
            0x31, // Futbol/Boca/CantidadDeGoles/2021
            0x00, 0x11, // Packet Identifier
            0x00, 0x05, 0x41, 0x4C, 0x54, 0x45, 0x47, // Payload 'ALTEG'
        ];
        stream_to_publish_message.write(&bytes_to_publish).unwrap();
        data = vec![0; 100];
        match stream_to_publish_message.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x40, 0x01, 0x00, 0x11]);
            }
            _ => {
                panic!();
            }
        }
        data = vec![0; 100];
        match stream_to_subscribe_with_wildcard.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], bytes_to_publish);
            }
            _ => {
                panic!();
            }
        }
        server.shutdown().unwrap();
    }

    #[test]
    fn testing_subscribe_and_publish_with_wildcard_double_astherisc() {
        let server = ServerTest::start("0.0.0.0:1893".to_string());

        let mut stream_to_create_topic_goles =
            TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();
        let mut stream_to_create_topic_partidos =
            TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();
        let mut stream_to_subscribe_with_wildcard =
            TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();
        let mut stream_to_publish_message = TcpStream::connect("0.0.0.0:1893".to_string()).unwrap();

        let subscriber_goles_connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x62, 0x63, // Client Identifier
        ];

        let subscriber_partidos_connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x00, 0x03, // Client Identifier
        ];

        let subscriber_with_wildcard_connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x00, 0x01, // Client Identifier
        ];

        let publisher_connect_bytes = [
            0x10, // Packet Type
            0x0E, // Remaining Length
            0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, // MQTT
            0x04, // Protocol Name - SIEMPRE en 04 o falla
            0x00, // Flags
            0x00, 0x0B, // keep alive
            0x00, 0x02, 0x00, 0x02, // Client Identifier
        ];

        stream_to_create_topic_goles
            .write(&subscriber_goles_connect_bytes)
            .unwrap();
        stream_to_create_topic_partidos
            .write(&subscriber_partidos_connect_bytes)
            .unwrap();
        stream_to_subscribe_with_wildcard
            .write(&subscriber_with_wildcard_connect_bytes)
            .unwrap();
        stream_to_publish_message
            .write(&publisher_connect_bytes)
            .unwrap();

        let mut data = vec![0; 100];
        match stream_to_create_topic_goles.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }
        data = vec![0; 100];
        match stream_to_subscribe_with_wildcard.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }
        data = vec![0; 100];
        match stream_to_publish_message.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }
        data = vec![0; 100];
        match stream_to_create_topic_partidos.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x20, 0x02, 0xFF, 0x00]);
            }
            _ => {
                panic!();
            }
        }

        let subscribe_bytes_crear_cantidad_de_goles = [
            0x80, // Packet Type
            0x20, // Remaining Length ---------> RECALCULAR PORQUE ESTA MAL
            0x00, 0x0A, // Variable Header, en particular packet identifier
            0x00, 0x1B, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
            0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64,
            0x64, 0x65, 0x67, 0x6F, 0x6C, 0x65, 0x73, // TOPIC: Futbol/Boca/CantidadDeGoles
            0x00,
        ];
        data = vec![0; 100];
        stream_to_create_topic_goles
            .write(&subscribe_bytes_crear_cantidad_de_goles)
            .unwrap();

        match stream_to_create_topic_goles.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x00]);
            }
            _ => {
                panic!();
            }
        }

        let subscribe_bytes_crear_cantidad_de_partidos = [
            0x80, // Packet Type
            0x23, // Remaining Length
            0x00, 0x0A, // Variable Header, en particular packet identifier
            0x00, 0x1E, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, 0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F,
            0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, 0x64, 0x65, 0x70, 0x61, 0x72, 0x74,
            0x69, 0x64, 0x6F, 0x73, // TOPIC: Futbol/Boca/CantidadDePartidos
            0x00,
        ];
        stream_to_create_topic_partidos
            .write(&subscribe_bytes_crear_cantidad_de_partidos)
            .unwrap();
        match stream_to_create_topic_partidos.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x00]);
            }
            _ => {
                panic!();
            }
        }

        let subscribe_bytes_with_wildcard = [
            0x80, // Packet Type
            0x0D, // Remaining Length
            0x00, 0x0A, // Variable Header, en particular packet identifier
            0x00, 0x08, 0x2A, // Inicio Payload
            0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x2A, // Topic :: */boca/*
            0x00,
        ];

        data = vec![0; 100];
        stream_to_subscribe_with_wildcard
            .write(&subscribe_bytes_with_wildcard)
            .unwrap();
        match stream_to_subscribe_with_wildcard.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x90, 0x03, 0x00, 0x0A, 0x00]);
            }
            _ => {
                panic!();
            }
        }
        let bytes_to_publish = [
            0x32, // Paquete publish
            0x26, // Remaining Length
            0x00, 0x1B, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // Inicio Payload
            0x2F, 0x62, 0x6F, 0x63, 0x61, 0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64,
            0x64, 0x65, 0x67, 0x6F, 0x6C, 0x65, 0x73, // Topic: Futbol/Boca/CantidadDeGOles
            0x00, 0x11, // Packet Identifier
            0x00, 0x05, 0x41, 0x4C, 0x54, 0x45, 0x47, // Payload 'ALTEG'
        ];

        stream_to_publish_message.write(&bytes_to_publish).unwrap();
        data = vec![0; 100];
        match stream_to_publish_message.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], [0x40, 0x01, 0x00, 0x11]);
            }
            _ => {
                panic!();
            }
        }

        data = vec![0; 100];
        match stream_to_subscribe_with_wildcard.read(&mut data) {
            Ok(size) => {
                assert_eq!(data[0..size], bytes_to_publish);
            }
            _ => {
                panic!();
            }
        }
        server.shutdown().unwrap();
    }
}
