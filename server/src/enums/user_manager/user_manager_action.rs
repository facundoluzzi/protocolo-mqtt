use crate::enums::user_manager::add_user_manager::AddUserManager;
use crate::enums::user_manager::disconnect_user_manager::DisconnectUserManager;
use crate::enums::user_manager::publish_message_user_manager::PublishMessageUserManager;
use crate::enums::user_manager::stop_publish_user_manager::StopPublish;

/// Enum de tipos para desencadenar eventos a traves de un channel de en el user manager.
pub enum UserManagerAction {
    AddUserManager(AddUserManager),
    DisconnectUserManager(DisconnectUserManager),
    PublishMessageUserManager(PublishMessageUserManager),
    StopPublishUserManager(StopPublish),
}
