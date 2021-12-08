use crate::usermanager::publishmessageusermanager::PublishMessageUserManager;
use crate::usermanager::disconnectusermanager::DisconnectUserManager;
use crate::usermanager::addusermanager::AddUserManager;

pub enum UserManagerAction {
    AddUserManager(AddUserManager),
    DisconnectUserManager(DisconnectUserManager),
    PublishMessageUserManager(PublishMessageUserManager),
}
