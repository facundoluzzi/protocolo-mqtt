use super::{
    add_autosend::AddAutoSend, publish_all_autosend::PublishAllAutoSend,
    remove_autosend::RemoveAutoSend,
};

pub enum AutoSendAction {
    Add(AddAutoSend),
    Remove(RemoveAutoSend),
    PublishAll(PublishAllAutoSend),
}
