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
use server::paquetes::subscribe::Unsubscribe;

#[test]
fn should_create_unsubscribe_packet() {
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
    assert_eq!(publisher_subscriber_sent.get_message(), "None".to_string());

    publisher_subscriber_sent
        .get_sender()
        .unwrap()
        .send((
            PublishMessageUserManager,
            "client_id".to_string(),
            None,
            None,
            Some("message".to_string()),
        ))
        .unwrap();

    let receiver_response = receiver_two.recv().unwrap();
    assert_eq!(receiver_response.1, "client_id".to_string());
    assert_eq!(receiver_response.4, Some("message".to_string()));
}