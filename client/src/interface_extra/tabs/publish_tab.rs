use crate::packet::{input::publish::Publish, sender_type::InterfaceSender};
use std::sync::mpsc::Sender;
extern crate gtk;
use gtk::prelude::*;

use crate::interface_extra::object_builder::{
    build_button_with_name, build_entry_with_name, build_radiobutton_with_name,
};

pub struct PublishTab {
    sender_of_client: Sender<InterfaceSender>,
}

impl PublishTab {
    pub fn new(sender_of_client: Sender<InterfaceSender>) -> Self {
        PublishTab { sender_of_client }
    }

    fn attach_action_for_publish_button(
        &self,
        publish_button: gtk::Button,
        message_input: gtk::Entry,
        topic_input: gtk::Entry,
        qos_publish_0: gtk::RadioButton,
        sender_publish: Sender<InterfaceSender>,
    ) {
        publish_button.connect_clicked(move |_| {
            let message = message_input.text().to_string();
            let topic = topic_input.text().to_string();
            let is_qos_0 = qos_publish_0.is_active();

            let publish = Publish::init(message, topic, is_qos_0);
            if let Err(_error) = sender_publish.send(InterfaceSender::Publish(publish)) {
                println!("Error en el publish");
            }
        });
    }

    pub fn build(&self, builder: &gtk::Builder) {
        let message_input: gtk::Entry = build_entry_with_name(builder, "message_input");
        let topic_input: gtk::Entry = build_entry_with_name(builder, "topic_input");

        let publish_button: gtk::Button = build_button_with_name(builder, "publish_button");

        let qos_publish_0: gtk::RadioButton = build_radiobutton_with_name(builder, "qos_publish_0");

        let sender_publish = self.get_clone_sender_of_client();

        self.attach_action_for_publish_button(
            publish_button,
            message_input,
            topic_input,
            qos_publish_0,
            sender_publish,
        );
    }

    fn get_clone_sender_of_client(&self) -> Sender<InterfaceSender> {
        self.sender_of_client.clone()
    }
}
