use super::{
    add_autosend::AddAutoSend, publish_all_autosend::PublishAllAutoSend,
    remove_autosend::RemoveAutoSend,
};

/// Enum de tipos para desencadenar eventos a traves de un channel de AutoSendAction en
/// por el que se maneja los packetes a los que no se les respondio el suback.
pub enum AutoSendAction {
    Add(AddAutoSend),
    Remove(RemoveAutoSend),
    PublishAll(PublishAllAutoSend),
    ChangeMode, 
}
