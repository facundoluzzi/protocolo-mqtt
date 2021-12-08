use crate::usermanager::add_user_manager::AddUserManager;
use crate::usermanager::disconnect_user_manager::DisconnectUserManager;
use crate::usermanager::publish_message_user_manager::PublishMessageUserManager;

pub enum UserManagerAction {
    AddUserManager(AddUserManager),
    DisconnectUserManager(DisconnectUserManager),
    PublishMessageUserManager(PublishMessageUserManager),
}
