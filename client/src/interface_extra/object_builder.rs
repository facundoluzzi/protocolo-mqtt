extern crate gtk;
use gtk::prelude::*;

pub fn build_entry_with_name(builder: &gtk::Builder, entry_name: &str) -> gtk::Entry {
    match builder.object(entry_name) {
        Some(entry) => entry,
        None => panic!("Can not create entry with name {}", entry_name),
    }
}

pub fn build_label_with_name(builder: &gtk::Builder, label_name: &str) -> gtk::Label {
    match builder.object(label_name) {
        Some(entry) => entry,
        None => panic!("Can not create entry with name {}", label_name),
    }
}

pub fn build_button_with_name(builder: &gtk::Builder, button_name: &str) -> gtk::Button {
    match builder.object(button_name) {
        Some(entry) => entry,
        None => panic!("Can not create entry with name {}", button_name),
    }
}

pub fn build_checkbutton_with_name(
    builder: &gtk::Builder,
    checkbutton_name: &str,
) -> gtk::CheckButton {
    match builder.object(checkbutton_name) {
        Some(entry) => entry,
        None => panic!("Can not create entry with name {}", checkbutton_name),
    }
}

pub fn build_radiobutton_with_name(
    builder: &gtk::Builder,
    radiobutton_name: &str,
) -> gtk::RadioButton {
    match builder.object(radiobutton_name) {
        Some(entry) => entry,
        None => panic!("Can not create entry with name {}", radiobutton_name),
    }
}
