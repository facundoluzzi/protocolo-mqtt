#[cfg(test)]
mod tests {
    use server::paquetes::publish::Publish;
    use server::paquetes::publisher_suscriber::PublisherSuscriber;
    use server::topics::topic_manager::TopicManager;
    use std::sync::mpsc;
    use std::sync::mpsc::Receiver;
    use std::sync::mpsc::Sender;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::thread;

    use server::helper::publisher_subscriber_code::PublisherSubscriberCode;
    use server::paquetes::subscribe::Subscribe;

    #[test]
    fn should_create_subscribe_packet() {
        let subscribe_bytes = [
            0x80, // packet type
            0x08, // remaining length
            0x00, 0x04,0x54, 0x54, 0x54, 0x54, // variable header, en particular packet identifier
            0x00, 0x04, 0x54, 0x54, 0x54, 0x54, 0x00, // payload TTTT como mensaje
        ];

        let (sender_one, receiver_one): (Sender<PublisherSuscriber>, Receiver<PublisherSuscriber>) =
            mpsc::channel();
        let (sender_two, receiver_two): (Sender<String>, Receiver<String>) = mpsc::channel();

        let messages: Vec<PublisherSuscriber> = Vec::new();
        let data = Arc::new(Mutex::new(messages));
        let data_for_thread = data.clone();
        
        let t = thread::spawn(move || {
            for recv in receiver_one.recv() {
                let mut data = data_for_thread.lock().unwrap();
                data.push(recv);
            }
        });

        let _subscribe = Subscribe::init(&subscribe_bytes).subscribe_topic(
            &sender_one,
            sender_two,
            "clientId".to_string(),
        );

        t.join().unwrap();

        let locked_data = data.lock().unwrap();
        let publisher_subscriber_sent = locked_data.get(0).unwrap();
        assert_eq!(
            publisher_subscriber_sent.get_packet_type(),
            PublisherSubscriberCode::Subscriber
        );
        assert_eq!(
            publisher_subscriber_sent.get_client_id(),
            "clientId".to_string()
        );

        let topic = publisher_subscriber_sent.get_topic();

        assert_eq!(topic, "TTTT".to_owned());
        assert_eq!(publisher_subscriber_sent.get_message(), "None".to_string());

        publisher_subscriber_sent
            .get_sender()
            .unwrap()
            .send("message".to_string())
            .unwrap();

        assert_eq!(receiver_two.recv().unwrap(), "message".to_string());
    }

    // #[test]
    // fn should_create_subscribe_packet_with_wilcard_astherisc_and_greater_than() {
    //     let publish_subscriber_sender = TopicManager::init();

    //     let subscribe_bytes = [
    //         0x80, // packet type
    //         0x1B, // remaining length ---------> RECALCULAR PORQUE ESTA MAL
    //         0x00, 0x0A, // variable header, en particular packet identifier
    //         0x00, 0x17, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // payload - futbol
    //         0x2F, 0x62, 0x6F, 0x63, 0x61, // boca
    //         0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, // cantidad
    //         0x64, 0x65, 0x2A, // de*     -> /cantidadDe*
    //         0x00, 
    //     ];

    //     let subscribe_bytes_crear_cantidad_de_goles = [
    //         0x80, // packet type
    //         0x1F, // remaining length ---------> RECALCULAR PORQUE ESTA MAL
    //         0x00, 0x0A, // variable header, en particular packet identifier
    //         0x00, 0x1B, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // payload - futbol
    //         0x2F, 0x62, 0x6F, 0x63, 0x61, // boca
    //         0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, // cantidad
    //         0x64, 0x65, 0x67, 0x6F, 0x6C, 0x65, 0x73, // de*     -> CANTIDAD DE GOLES
    //         0x00, 
    //     ];

    //     let subscribe_bytes_crear_cantidad_de_partidos = [
    //         0x80, // packet type
    //         0x22, // remaining length ---------> RECALCULAR PORQUE ESTA MAL
    //         0x00, 0x0A, // variable header, en particular packet identifier
    //         0x00, 0x1E, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, 
    //         0x2F, 0x62, 0x6F, 0x63, 0x61, 
    //         0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, 
    //         0x64, 0x65, 0x70, 0x61, 0x72, 0x74, 0x69, 0x64, 0x6F, 0x73,// CANTIDAD DE PARTIDOS
    //         0x00, 
    //     ];

    //     let (sender_two, receiver_two): (Sender<String>, Receiver<String>) = mpsc::channel();

    //     let (sender_partidos, _receiver_partidos): (Sender<String>, Receiver<String>) = mpsc::channel();
    //     let (sender_goles, _receiver_goles): (Sender<String>, Receiver<String>) = mpsc::channel();

    //     let _subscribe_partidos = Subscribe::init(&subscribe_bytes_crear_cantidad_de_partidos).subscribe_topic(
    //         &publish_subscriber_sender,
    //         sender_partidos,
    //         "CREARPARTIDOS".to_string(),
    //     );

    //     let _subscribe_goles = Subscribe::init(&subscribe_bytes_crear_cantidad_de_goles).subscribe_topic(
    //         &publish_subscriber_sender,
    //         sender_goles,
    //         "CREARGOLES".to_string(),
    //     );

    //     let _subscribe = Subscribe::init(&subscribe_bytes).subscribe_topic(
    //         &publish_subscriber_sender,
    //         sender_two,
    //         "clientId".to_string(),
    //     );

    //     let bytes = [
    //         0x32, // Paquete publish
    //         0x29, // Remaining Length
    //         0x00, 0x1F, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // payload - futbol
    //         0x2F, 0x62, 0x6F, 0x63, 0x61, // boca
    //         0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, // cantidad
    //         0x64, 0x65, 0x70, 0x61, 0x72, 0x74, 0x69, 0x64, 0x6F, 0x73, 
    //         0x00, 0x10, // Packet Identifier
    //         0x00, 0x05, 0x41, 0x4C, 0x54, 0x45, 0x47, // Payload 'Message'
    //     ];

    //     let publish = Publish::init(&bytes);
    //     let topic = publish.get_topic();
    //     assert_ne!(topic, "futbol/boca/cantidaddepartidos".to_owned());
    //     println!("AAAAAAAAAAAAAAAAAAAA");
    //     assert_eq!(receiver_two.recv().unwrap(), "Message");
    //     // assert_eq!(receiver_two.recv().unwrap(), "message".to_string());
    // }


}

// mod test {
//     #[test] 
//     pub fn calcular() {

//         let subscribe_bytes = [
//             0x80, // packet type
//             0x1B, // remaining length ---------> RECALCULAR PORQUE ESTA MAL
//             0x00, 0x0A, // variable header, en particular packet identifier
//             0x00, 0x17, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // payload - futbol
//             0x2F, 0x62, 0x6F, 0x63, 0x61, // boca
//             0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, // cantidad
//             0x64, 0x65, 0x2A, // de*     -> /cantidadDe*
//             0x00, 
//         ];

//         let subscribe_bytes_crear_cantidad_de_goles = [
//             0x80, // packet type
//             0x1F, // remaining length ---------> RECALCULAR PORQUE ESTA MAL
//             0x00, 0x0A, // variable header, en particular packet identifier
//             0x00, 0x1B, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // payload - futbol
//             0x2F, 0x62, 0x6F, 0x63, 0x61, // boca
//             0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, // cantidad
//             0x64, 0x65, 0x67, 0x6F, 0x6C, 0x65, 0x73, // de*     -> CANTIDAD DE GOLES
//             0x00, 
//         ];

//         let subscribe_bytes_crear_cantidad_de_partidos = [
//             0x80, // packet type
//             0x22, // remaining length ---------> RECALCULAR PORQUE ESTA MAL
//             0x00, 0x0A, // variable header, en particular packet identifier
//             0x00, 0x1E, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, 
//             0x2F, 0x62, 0x6F, 0x63, 0x61, 
//             0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, 
//             0x64, 0x65, 0x70, 0x61, 0x72, 0x74, 0x69, 0x64, 0x6F, 0x73,// CANTIDAD DE PARTIDOS
//             0x00, 
//         ];

//         let vector: Vec<String> =  "0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // payload - futbol
//         0x2F, 0x62, 0x6F, 0x63, 0x61, // boca
//         0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, // cantidad
//         0x64, 0x65, 0x2A"
//         .split(',').map(|s| s.to_string()).collect();

//         println!("\n\nLongitud: {:?}", vector.len());
//     }
// }