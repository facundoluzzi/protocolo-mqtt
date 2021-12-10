use crate::enums::user_manager::add_user_manager::AddUserManager;
use crate::enums::user_manager::disconnect_user_manager::DisconnectUserManager;
use crate::enums::user_manager::publish_message_user_manager::PublishMessageUserManager;

pub enum UserManagerAction {
    AddUserManager(AddUserManager),
    DisconnectUserManager(DisconnectUserManager),
    PublishMessageUserManager(PublishMessageUserManager),
}
