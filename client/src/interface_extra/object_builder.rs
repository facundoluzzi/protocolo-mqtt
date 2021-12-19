extern crate gtk;
use gtk::prelude::*;

/// Construye un objeto Entry de GTK que corresponde con el ID del objeto y lo devuelve
pub fn build_entry_with_name(builder: &gtk::Builder, entry_name: &str) -> gtk::Entry {
    match builder.object(entry_name) {
        Some(entry) => entry,
        None => panic!("Can not create entry with name {}", entry_name),
    }
}

/// Construye un objeto Label de GTK que corresponde con el ID del objeto y lo devuelve
pub fn build_label_with_name(builder: &gtk::Builder, label_name: &str) -> gtk::Label {
    match builder.object(label_name) {
        Some(entry) => entry,
        None => panic!("Can not create entry with name {}", label_name),
    }
}

/// Construye un objeto Button de GTK que corresponde con el ID del objeto y lo devuelve
pub fn build_button_with_name(builder: &gtk::Builder, button_name: &str) -> gtk::Button {
    match builder.object(button_name) {
        Some(entry) => entry,
        None => panic!("Can not create entry with name {}", button_name),
    }
}

/// Construye un objeto CheckButton de GTK que corresponde con el ID del objeto y lo devuelve
pub fn build_checkbutton_with_name(
    builder: &gtk::Builder,
    checkbutton_name: &str,
) -> gtk::CheckButton {
    match builder.object(checkbutton_name) {
        Some(entry) => entry,
        None => panic!("Can not create entry with name {}", checkbutton_name),
    }
}

/// Construye un objeto RadioButton de GTK que corresponde con el ID del objeto y lo devuelve
pub fn build_radiobutton_with_name(
    builder: &gtk::Builder,
    radiobutton_name: &str,
) -> gtk::RadioButton {
    match builder.object(radiobutton_name) {
        Some(entry) => entry,
        None => panic!("Can not create entry with name {}", radiobutton_name),
    }
}
