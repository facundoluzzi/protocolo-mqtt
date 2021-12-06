use server::paquetes::publisher_suscriber::PublisherSuscriber;
use server::usermanager::user_manager_action::UserManagerAction::PublishMessageUserManager;
use server::usermanager::user_manager_types::ChannelUserManager;
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
        0x09, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
        0x00, 0x03, 0x61, 0x2F, 0x62, 0x01, // payload MQTT como mensaje + qos
    ];

    let (sender_one, receiver_one): (Sender<PublisherSuscriber>, Receiver<PublisherSuscriber>) =
        mpsc::channel();

    let (sender_two, receiver_two): (Sender<ChannelUserManager>, Receiver<ChannelUserManager>) =
        mpsc::channel();

    let messages: Vec<PublisherSuscriber> = Vec::new();
    let data = Arc::new(Mutex::new(messages));
    let data_for_thread = data.clone();

    let t = thread::spawn(move || {
        for recv in receiver_one.recv() {
            let mut data = data_for_thread.lock().unwrap();
            data.push(recv);
        }
    });

    let _subscribe = Subscribe::init(&subscribe_bytes).unwrap().subscribe_topic(
        sender_one,
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

    assert_eq!(topic, "a/b".to_owned());
    assert_eq!(publisher_subscriber_sent.get_publish_packet(), None);

    publisher_subscriber_sent
        .get_sender()
        .unwrap()
        .send((
            PublishMessageUserManager,
            "client_id".to_string(),
            None,
            None,
            Some([0x00, 0x01, 0x02].to_vec()),
        ))
        .unwrap();

    let receiver_response = receiver_two.recv().unwrap();
    assert_eq!(receiver_response.1, "client_id".to_string());
    assert_eq!(receiver_response.4, Some([0x00, 0x01, 0x02].to_vec()));
}

// #[test]
// fn should_create_subscribe_packet_with_wilcard_astherisc_and_greater_than() {

//     let (publish_subscriber_sender, publisher_subscriber_receiver): (Sender<PublisherSuscriber>, Receiver<PublisherSuscriber>) =
//         mpsc::channel();

//     let topic_boca_goles = Topic::new("futbol/boca/cantidaddegoles".to_owned());
//     let topic_boca_partidos = Topic::new("futbol/boca/cantidaddepartidos".to_owned());
//     let mut topics: HashMap<String, Sender<SenderTopicType>> = HashMap::new();
//     topics.insert("futbol/boca/cantidaddegoles".to_owned(), topic_boca_goles);
//     topics.insert("futbol/boca/cantidaddepartidos".to_owned(), topic_boca_partidos);

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

//     let (sender_two, receiver_two): (Sender<String>, Receiver<String>) = mpsc::channel();

//     std::thread::spawn(move || {
//         for publisher_subscriber in publisher_subscriber_receiver {
//             match publisher_subscriber.get_packet_type() {
//                 PublisherSubscriberCode::Publisher => {
//                     if let Some(topic_sender) = topics.get(&publisher_subscriber.get_topic()) {
//                         topic_sender.send((PublishMessage, publisher_subscriber.get_message(), None)).unwrap();
//                     }
//                 },
//                 PublisherSubscriberCode::Subscriber => {
//                     let subscriber = publisher_subscriber.get_sender();
//                     let topic_name = publisher_subscriber.get_topic();
//                     let client_id = publisher_subscriber.get_client_id();
//                     if let Some(wilcard) = verify_wildcard::get_wilcard(topic_name.to_owned()){
//                         for (topic_name, topic_sender) in &topics {
//                             if wilcard.verify_topic(topic_name.to_owned()) {
//                                 topic_sender.send((AddTopic, client_id.to_owned(), subscriber.clone())).unwrap();
//                             }
//                         }
//                     } else {
//                         assert_eq!(0, 1);
//                     }
//                 }
//             };
//         }
//         for rx in receiver_two {
//             assert_eq!(rx, "ALTEG".to_owned())
//         }
//     });

//     let _subscribe = Subscribe::init(&subscribe_bytes).subscribe_topic(
//         &publish_subscriber_sender,
//         sender_two,
//         "clientId".to_string(),
//     );

//     let publisher_subscriber = receiver.recv().unwrap();

//     assert_eq!(publisher_subscriber.get_algo(), 'algo');

//     let bytes = [
//         0x32, // Paquete publish
//         0x29, // Remaining Length
//         0x00, 0x1E, 0x66, 0x75, 0x74, 0x62, 0x6F, 0x6C, // payload - futbol
//         0x2F, 0x62, 0x6F, 0x63, 0x61, // boca
//         0x2F, 0x63, 0x61, 0x6E, 0x74, 0x69, 0x64, 0x61, 0x64, // cantidad
//         0x64, 0x65, 0x70, 0x61, 0x72, 0x74, 0x69, 0x64, 0x6F, 0x73,
//         0x00, 0x10, // Packet Identifier
//         0x00, 0x05, 0x41, 0x4C, 0x54, 0x45, 0x47, // Payload 'Message'
//     ];

//     let publish = Publish::init(&bytes);
//     publish.send_message(&publish_subscriber_sender, "client_id".to_owned());
// }
