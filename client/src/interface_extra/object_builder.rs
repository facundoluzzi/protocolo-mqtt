fn build_entry_with_name(&self, builder: &gtk::Builder, entry_name: &str) -> gtk::Entry {
    match builder.object(entry_name) {
        Some(entry) => entry,
        None => panic!("Can not create entry with name {}", entry_name),
    }
}

fn build_label_with_name(&self, builder: &gtk::Builder, label_name: &str) -> gtk::Label {
    match builder.object(label_name) {
        Some(entry) => entry,
        None => panic!("Can not create entry with name {}", label_name),
    }
}

fn build_button_with_name(&self, builder: &gtk::Builder, button_name: &str) -> gtk::Button {
    match builder.object(button_name) {
        Some(entry) => entry,
        None => panic!("Can not create entry with name {}", button_name),
    }
}

fn build_checkbutton_with_name(
    &self,
    builder: &gtk::Builder,
    checkbutton_name: &str,
) -> gtk::CheckButton {
    match builder.object(checkbutton_name) {
        Some(entry) => entry,
        None => panic!("Can not create entry with name {}", checkbutton_name),
    }
}

fn build_radiobutton_with_name(
    &self,
    builder: &gtk::Builder,
    radiobutton_name: &str,
) -> gtk::RadioButton {
    match builder.object(radiobutton_name) {
        Some(entry) => entry,
        None => panic!("Can not create entry with name {}", radiobutton_name),
    }
}