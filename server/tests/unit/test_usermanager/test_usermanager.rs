use std::sync::mpsc::Sender;


#[cfg(test)]
mod tests {
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use server::usermanager::user_manager::UserManager;
use server::usermanager::user_manager_action::UserManagerAction::GetSenderUserManager;
use server::usermanager::user_manager_action::UserManagerAction::AddUserManager;
use server::usermanager::user_manager_action::UserManagerAction::DeleteUserManager;
use server::usermanager::user_manager_action::UserManagerAction::DisconectUserManager;
use server::stream::stream_handler::StreamType;

    use super::*;

    #[test]
    fn should_add_a_user_and_getSender() {
        let sender = UserManager::init();
        let (sender_extraction, receiver_extraction): (Sender<Sender<String>>, Receiver<Sender<String>>) = mpsc::channel();
        let (sender_stream, receiver_stream): (Sender<StreamType>, Receiver<StreamType>) = mpsc::channel();
        sender.send((AddUserManager,"Nacho".to_owned(),Some(sender_stream), Some(true),None)).unwrap();
        sender.send((GetSenderUserManager,"Nacho".to_owned(),None, None,Some(sender_extraction))).unwrap();
        match  receiver_extraction.recv(){
            Ok(_) =>{ assert_eq!(1,1) }
            Err(_) =>{ assert_eq!(0,1) }
        }
    }
    #[test]
    fn should_add_a_user_and_remove_cant_get_sender() {
        let sender = UserManager::init();
        let (sender_extraction, receiver_extraction): (Sender<Sender<String>>, Receiver<Sender<String>>) = mpsc::channel();
        let (sender_stream, receiver_stream): (Sender<StreamType>, Receiver<StreamType>) = mpsc::channel();
        sender.send((AddUserManager,"Nacho".to_owned(),Some(sender_stream), Some(true),None)).unwrap();
        sender.send((DeleteUserManager,"Nacho".to_owned(),None,None,None)).unwrap();
        sender.send((GetSenderUserManager,"Nacho".to_owned(),None, None,Some(sender_extraction))).unwrap();
        match  receiver_extraction.recv(){
            Err(_) =>{ assert_eq!(1,1) }
            Ok(_) =>{ assert_eq!(0,1) }
        }
    }
    #[test]
    fn should_add_a_user_and_disconnect_get_sender_give_none() {
        let sender = UserManager::init();
        let (sender_extraction, receiver_extraction): (Sender<Sender<String>>, Receiver<Sender<String>>) = mpsc::channel();
        let (sender_stream, receiver_stream): (Sender<StreamType>, Receiver<StreamType>) = mpsc::channel();
        sender.send((AddUserManager,"Nacho".to_owned(),Some(sender_stream), Some(false),None)).unwrap();
        sender.send((GetSenderUserManager,"Nacho".to_owned(),None, None,Some(sender_extraction.clone()))).unwrap();
        match  receiver_extraction.recv(){
            Ok(_) =>{ assert_eq!(1,1) }
            Err(_) =>{ assert_eq!(0,1) }
        }
        sender.send((DisconectUserManager,"Nacho".to_owned(),None,None,None)).unwrap();
        sender.send((GetSenderUserManager,"Nacho".to_owned(),None, None,Some(sender_extraction))).unwrap();
        match  receiver_extraction.recv(){
            Ok(_) =>{ assert_eq!(0,1) }
            Err(_) =>{ assert_eq!(1,1) }
        }
    }
    #[test]
    fn should_add_a_user_and_disconnect_and_reconnect_give_some_sender() {
        let sender = UserManager::init();
        let (sender_extraction, receiver_extraction): (Sender<Sender<String>>, Receiver<Sender<String>>) = mpsc::channel();
        let (sender_stream, receiver_stream): (Sender<StreamType>, Receiver<StreamType>) = mpsc::channel();
        sender.send((AddUserManager,"Nacho".to_owned(),Some(sender_stream), Some(false),None)).unwrap();
        sender.send((GetSenderUserManager,"Nacho".to_owned(),None, None,Some(sender_extraction.clone()))).unwrap();
        match  receiver_extraction.recv(){
            Ok(_) =>{ assert_eq!(1,1) }
            Err(_) =>{ assert_eq!(0,1) }
        }
        sender.send((DisconectUserManager,"Nacho".to_owned(),None,None,None)).unwrap();
        sender.send((GetSenderUserManager,"Nacho".to_owned(),None, None,Some(sender_extraction))).unwrap();
        match  receiver_extraction.recv(){
            Ok(_) =>{ assert_eq!(0,1) }
            Err(_) =>{ assert_eq!(1,1) }
        }
        sender.send((AddUserManager,"Nacho".to_owned(),Some(sender_stream), Some(false),None)).unwrap();
        sender.send((GetSenderUserManager,"Nacho".to_owned(),None, None,Some(sender_extraction.clone()))).unwrap();
        match  receiver_extraction.recv(){
            Ok(_) =>{ assert_eq!(1,1) }
            Err(_) =>{ assert_eq!(0,1) }
        }

    }
} 