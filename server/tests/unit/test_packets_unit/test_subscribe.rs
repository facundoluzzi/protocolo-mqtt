use server::topics::topic_types::TypeTopicManager;
use server::topics::topic_types::TypeTopicManager::Subscriber;
use server::usermanager::publish_message_user_manager::PublishMessageUserManager;
use server::usermanager::user_manager_action::UserManagerAction;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use server::paquetes::subscribe::Subscribe;

#[test]
fn should_create_subscribe_packet() {
    let subscribe_bytes = [
        0x80, // packet type
        0x09, // remaining length
        0x00, 0x0A, // variable header, en particular packet identifier
        0x00, 0x03, 0x61, 0x2F, 0x62, 0x01, // payload MQTT como mensaje + qos
    ];

    let (sender_one, receiver_one): (Sender<TypeTopicManager>, Receiver<TypeTopicManager>) =
        mpsc::channel();

    let (sender_two, receiver_two): (Sender<UserManagerAction>, Receiver<UserManagerAction>) =
        mpsc::channel();

    let messages: Vec<TypeTopicManager> = Vec::new();
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
    let type_topic_manager = locked_data.get(0).unwrap();

    match type_topic_manager {
        Subscriber(subscriber) => {
            assert_eq!(subscriber.get_client_id(), "clientId".to_string());

            let topic = subscriber.get_topic();

            assert_eq!(topic, "a/b".to_owned());
            let action =
                UserManagerAction::PublishMessageUserManager(PublishMessageUserManager::init(
                    "client_id".to_owned(),
                    [0x00, 0x01, 0x02].to_vec(),
                ));
            subscriber.get_sender_user_manager().send(action).unwrap();

            match receiver_two.recv().unwrap() {
                UserManagerAction::PublishMessageUserManager(user) => {
                    assert_eq!(user.get_client_id(), "client_id".to_string());
                    assert_eq!(user.get_message(), [0x00, 0x01, 0x02].to_vec());
                }
                _ => assert_eq!(0, 1),
            }
        }
        _ => {
            panic!("unexpected error");
        }
    }
}
