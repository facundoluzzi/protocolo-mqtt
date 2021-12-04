// use core::time;
// use std::{net::TcpListener, thread};

// use server::{
//     logs::logger::Logger, server::main::run_server, topics::topic_manager::TopicManager,
//     usermanager::user_manager::UserManager,
// };

// fn setup() {
//     match TcpListener::bind("0.0.0.0:1883") {
//         Ok(listener) => {
//             thread::spawn(move || {
//                 let sender_user_manager = UserManager::init();
//                 let logger = Logger::new("connect-tests.txt".to_string())
//                     .expect("Logger could not be created");
//                 let sender_topic_manager = TopicManager::init();
//                 run_server(&listener, logger, sender_topic_manager, sender_user_manager);
//             });
//             thread::sleep(time::Duration::from_millis(100));
//         }
//         Err(_) => {}
//     }
// }

// mod tests {
//     use std::time::Duration;
//     use std::{
//         io::{Read, Write},
//         net::TcpStream,
//         thread,
//     };

//     use super::setup;

//     #[test]
//     fn probando() {
//         // setup();
//         // let mut stream_client_1 = TcpStream::connect("localhost:1883").unwrap();
//         // let connect_bytes_client_1 = [
//         //     0x10, // packet type
//         //     0x0E, // remaining length
//         //     0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
//         //     0x03, // protocol name
//         //     0x00, // flags
//         //     0x00, 0x0B, // keep alive
//         //     0x00, 0x02, 0x00, 0x00, // client identifier
//         // ];

//         // let mut stream_client_2 = TcpStream::connect("localhost:1883").unwrap();
//         // let connect_bytes_client_2 = [
//         //     0x10, // packet type
//         //     0x0E, // remaining length
//         //     0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
//         //     0x03, // protocol name
//         //     0x00, // flags
//         //     0x00, 0x0B, // keep alive
//         //     0x00, 0x02, 0x00, 0x01, // client identifier
//         // ];

//         // let mut stream_client_3 = TcpStream::connect("localhost:1883").unwrap();
//         // let connect_bytes_client_3 = [
//         //     0x10, // packet type
//         //     0x0E, // remaining length
//         //     0x00, 0x04, 0x4D, 0x15, 0x45, 0x45, //mqtt
//         //     0x03, // protocol name
//         //     0x00, // flags
//         //     0x00, 0x0B, // keep alive
//         //     0x00, 0x02, 0x00, 0x02, // client identifier
//         // ];

//         // stream_client_1.write(&connect_bytes_client_1).unwrap();
//         // stream_client_2.write(&connect_bytes_client_2).unwrap();
//         // stream_client_3.write(&connect_bytes_client_3).unwrap();
//         // thread::sleep(time::Duration::from_millis(100));

//         // let mut data1 = vec![0; 100];
//         // match stream_client_1.read(&mut data1) {
//         //     Ok(size) => {
//         //         assert_eq!(data1[0..size], [0x20, 0x02, 0xFF, 0x01]);
//         //     }
//         //     _ => {
//         //         panic!();
//         //     }
//         // }
//         // let mut data2 = vec![0; 100];
//         // match stream_client_2.read(&mut data2) {
//         //     Ok(size) => {
//         //         assert_eq!(data2[0..size], [0x20, 0x02, 0xFF, 0x01]);
//         //     }
//         //     _ => {
//         //         panic!();
//         //     }
//         // }
//         // let mut data3 = vec![0; 100];
//         // match stream_client_3.read(&mut data3) {
//         //     Ok(size) => {
//         //         assert_eq!(data3[0..size], [0x20, 0x02, 0xFF, 0x01]);
//         //     }
//         //     _ => {
//         //         panic!();
//         //     }
//         // }
//         // let subscribe_bytes_crear_cantidad_de_goles = [
//         //     0x80, // packet type
//         //     0x1F, // remaining length ---------> RECALCULAR PORQUE ESTA MAL
//         //     0x00, 0x0A, // variable header, en particular packet identifier
//         //     0x00, 0x1B, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // payload - futbol
//         //     0x2F, 0x62, 0x6F, 0x63, 0x61, // boca
//         //     0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, // cantidad
//         //     0x64, 0x65, 0x67, 0x6F, 0x6C, 0x65, 0x73, // de*     -> CANTIDAD DE GOLES
//         //     0x00,
//         // ];
//         // let subscribe_bytes_crear_cantidad_de_partidos = [
//         //     0x80, // packet type
//         //     0x22, // remaining length ---------> RECALCULAR PORQUE ESTA MAL
//         //     0x00, 0x0A, // variable header, en particular packet identifier
//         //     0x00, 0x1E, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C,
//         //     0x2F, 0x62, 0x6F, 0x63, 0x61,
//         //     0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64,
//         //     0x64, 0x65, 0x70, 0x61, 0x72, 0x74, 0x69, 0x64, 0x6F, 0x73,// CANTIDAD DE PARTIDOS
//         //     0x00,
//         // ];
//         // stream_client_1.write(&subscribe_bytes_crear_cantidad_de_goles).unwrap();
//         // stream_client_1.write(&subscribe_bytes_crear_cantidad_de_partidos).unwrap();

//         // let subscribe_bytes = [
//         //     0x80, // packet type
//         //     0x1B, // remaining length ---------> RECALCULAR PORQUE ESTA MAL
//         //     0x00, 0x0A, // variable header, en particular packet identifier
//         //     0x00, 0x17, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // payload - futbol
//         //     0x2F, 0x62, 0x6F, 0x63, 0x61, // boca
//         //     0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, // cantidad
//         //     0x64, 0x65, 0x2A, // de*     -> /cantidadDe*
//         //     0x00,
//         // ];
//         // let mut data2 = vec![0; 100];
//         // stream_client_2.write(&subscribe_bytes).unwrap();
//         // match stream_client_2.read(&mut data2) {
//         //     Ok(_size) => {
//         //     }
//         //     _ => {
//         //         panic!();
//         //     }
//         // }

//         // let bytes_to_publish = [
//         //     0x32, // Paquete publish
//         //     0x11, // Remaining Length
//         //     0x00, 0x06, 0x41, 0x4C, 0x54, 0x45, 0x47, 0x4F, // Topic 'ALTEGO'
//         //     0x00, 0x11, // Packet Identifier
//         //     0x00, 0x05, 0x41, 0x4C, 0x54, 0x45, 0x47, // Payload 'ALTEG'
//         // ];
//         // stream_client_3.write(&bytes_to_publish).unwrap();
//         // let mut data2 = vec![0; 100];
//         // match stream_client_2.read(&mut data2) {
//         //     Ok(size) => {
//         //         let buf = &data2[0..size];
//         //         assert_eq!(std::str::from_utf8(buf).unwrap(), "ALTEG".to_owned());
//         //     }
//         //     _ => {
//         //         panic!();
//         //     }
//         // }
//     }
// }
