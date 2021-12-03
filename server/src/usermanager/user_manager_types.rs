use crate::stream::stream_handler::StreamType;
use crate::usermanager::user_manager_action::UserManagerAction;
use std::sync::mpsc::Sender;

pub type ChannelUserManager = (
    UserManagerAction,
    String,
    Option<Sender<StreamType>>,
    Option<bool>,
    Option<String>,
);
