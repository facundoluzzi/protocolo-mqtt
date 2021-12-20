use server::enums::user_manager::add_user_manager::AddUserManager;
use server::enums::user_manager::user_manager_action::UserManagerAction;
use server::helper::status_code::ConnectReturnCode;
use server::stream::stream_handler::StreamType;
use server::topic::topic_manager::TopicManager;
use server::usermanager::user_manager::UserManager;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

#[test]
fn return_code_is_success() {
    let connect_return_code = ConnectReturnCode::init();
    assert_eq!(connect_return_code.apply_validations(), 0x00);
}

#[test]
fn return_code_is_success_complete_validation() {
    let sender_topic_manager = TopicManager::init();
    let sender_user_manager = UserManager::init(sender_topic_manager);

    let connect_return_code = ConnectReturnCode::init()
        .check_protocol_level(4)
        .check_client_id(Some("client_id".to_string()), sender_user_manager)
        .check_malformed_username("user1".to_string())
        .check_malformed_password("pass1".to_string())
        .check_authentication(Some("user1".to_string()), Some("pass1".to_string()));
    assert_eq!(connect_return_code.apply_validations(), 0x00);
}

#[test]
fn return_code_is_none_client_id() {
    let sender_topic_manager = TopicManager::init();
    let sender_user_manager = UserManager::init(sender_topic_manager);

    let connect_return_code = ConnectReturnCode::init()
        .check_protocol_level(4)
        .check_client_id(None, sender_user_manager.clone())
        .check_malformed_username("user1".to_string())
        .check_malformed_password("pass1".to_string())
        .check_authentication(Some("user1".to_string()), Some("pass1".to_string()));
    assert_eq!(connect_return_code.apply_validations(), 0x02);
}

#[test]
fn return_code_is_invalid_client_id() {
    let sender_topic_manager = TopicManager::init();
    let sender_user_manager = UserManager::init(sender_topic_manager);

    let connect_return_code = ConnectReturnCode::init()
        .check_protocol_level(4)
        .check_client_id(Some("client_id".to_string()), sender_user_manager.clone())
        .check_malformed_username("user1".to_string())
        .check_malformed_password("pass1".to_string())
        .check_authentication(Some("user1".to_string()), Some("pass1".to_string()));
    assert_eq!(connect_return_code.apply_validations(), 0x00);

    let (sender_stream, _receiver_stream): (Sender<StreamType>, Receiver<StreamType>) = channel();
    let action_to_add = UserManagerAction::AddUserManager(AddUserManager::init_without_will(
        "client_id".to_owned(),
        sender_stream,
        true,
    ));
    sender_user_manager.send(action_to_add).unwrap();

    let connect_return_code = ConnectReturnCode::init()
        .check_protocol_level(4)
        .check_client_id(Some("client_id".to_string()), sender_user_manager)
        .check_malformed_username("user1".to_string())
        .check_malformed_password("pass1".to_string())
        .check_authentication(Some("user1".to_string()), Some("pass1".to_string()));
    assert_eq!(connect_return_code.apply_validations(), 0x02);
}

#[test]
fn return_code_is_unacceptable_protocol() {
    let sender_topic_manager = TopicManager::init();
    let sender_user_manager = UserManager::init(sender_topic_manager);

    let connect_return_code = ConnectReturnCode::init()
        .check_protocol_level(2)
        .check_client_id(Some("client_id".to_string()), sender_user_manager)
        .check_malformed_username("user1".to_string())
        .check_malformed_password("pass1".to_string())
        .check_authentication(Some("user1".to_string()), Some("pass1".to_string()));
    assert_eq!(connect_return_code.apply_validations(), 0x01);
}

#[test]
fn return_code_is_malformed_username() {
    let sender_topic_manager = TopicManager::init();
    let sender_user_manager = UserManager::init(sender_topic_manager);

    let connect_return_code = ConnectReturnCode::init()
        .check_protocol_level(4)
        .check_client_id(Some("client_id".to_string()), sender_user_manager)
        .check_malformed_username("".to_string())
        .check_malformed_password("pass1".to_string())
        .check_authentication(Some("user1".to_string()), Some("pass1".to_string()));
    assert_eq!(connect_return_code.apply_validations(), 0x04);
}

#[test]
fn return_code_is_malformed_password() {
    let sender_topic_manager = TopicManager::init();
    let sender_user_manager = UserManager::init(sender_topic_manager);

    let connect_return_code = ConnectReturnCode::init()
        .check_protocol_level(4)
        .check_client_id(Some("client_id".to_string()), sender_user_manager)
        .check_malformed_username("213".to_string())
        .check_malformed_password("".to_string())
        .check_authentication(Some("user1".to_string()), Some("pass1".to_string()));
    assert_eq!(connect_return_code.apply_validations(), 0x04);
}

#[test]
fn return_code_is_not_authorized_wrong_username() {
    let sender_topic_manager = TopicManager::init();
    let sender_user_manager = UserManager::init(sender_topic_manager);

    let connect_return_code = ConnectReturnCode::init()
        .check_protocol_level(4)
        .check_client_id(Some("client_id".to_string()), sender_user_manager)
        .check_malformed_username("user1".to_string())
        .check_malformed_password("pass1".to_string())
        .check_authentication(Some("user".to_string()), Some("pass1".to_string()));
    assert_eq!(connect_return_code.apply_validations(), 0x05);
}

#[test]
fn return_code_is_not_authorized_wrong_password() {
    let sender_topic_manager = TopicManager::init();
    let sender_user_manager = UserManager::init(sender_topic_manager);

    let connect_return_code = ConnectReturnCode::init()
        .check_protocol_level(4)
        .check_client_id(Some("client_id".to_string()), sender_user_manager)
        .check_malformed_username("user1".to_string())
        .check_malformed_password("pass1".to_string())
        .check_authentication(Some("user".to_string()), Some("pass1".to_string()));
    assert_eq!(connect_return_code.apply_validations(), 0x05);
}

#[test]
fn return_code_is_success_password_empty() {
    let sender_topic_manager = TopicManager::init();
    let sender_user_manager = UserManager::init(sender_topic_manager);

    let connect_return_code = ConnectReturnCode::init()
        .check_protocol_level(4)
        .check_client_id(Some("client_id".to_string()), sender_user_manager)
        .check_malformed_username("user1".to_string())
        .check_malformed_password("pass1".to_string())
        .check_authentication(None, None);
    assert_eq!(connect_return_code.apply_validations(), 0x00);
}

#[test]
fn return_code_is_not_authorized_empty_password() {
    let sender_topic_manager = TopicManager::init();
    let sender_user_manager = UserManager::init(sender_topic_manager);

    let connect_return_code = ConnectReturnCode::init()
        .check_protocol_level(4)
        .check_client_id(Some("client_id".to_string()), sender_user_manager)
        .check_malformed_username("user1".to_string())
        .check_malformed_password("pass1".to_string())
        .check_authentication(Some("user1".to_string()), None);
    assert_eq!(connect_return_code.apply_validations(), 0x05);
}

#[test]
fn return_code_is_not_authorized_empty_username() {
    let sender_topic_manager = TopicManager::init();
    let sender_user_manager = UserManager::init(sender_topic_manager);

    let connect_return_code = ConnectReturnCode::init()
        .check_protocol_level(4)
        .check_client_id(Some("client_id".to_string()), sender_user_manager)
        .check_malformed_username("user1".to_string())
        .check_malformed_password("pass1".to_string())
        .check_authentication(None, Some("pass1".to_string()));
    assert_eq!(connect_return_code.apply_validations(), 0x05);
}
