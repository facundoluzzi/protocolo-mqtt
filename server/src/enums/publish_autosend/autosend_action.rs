use super::{remove_autosend::RemoveAutoSend, add_autosend::AddAutoSend};

pub enum AutoSendAction {
    Add(AddAutoSend),
    Remove(RemoveAutoSend),
}
