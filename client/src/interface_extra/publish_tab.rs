use crate::packet::{input::publish::Publish, sender_type::InterfaceSender};
use std::sync::mpsc::Sender;
extern crate gtk;
use gtk::prelude::*;

pub struct PublishTab {
    sender_of_client: Sender<InterfaceSender>,
}

impl PublishTab {
    pub fn new(sender_of_client: Sender<InterfaceSender>) -> Self {
        PublishTab { sender_of_client }
    }

    pub fn build(&self, builder: &gtk::Builder) {
        let message_input: gtk::Entry = self.build_entry_with_name(&builder, "message_input");
        let topic_input: gtk::Entry = self.build_entry_with_name(&builder, "topic_input");

        let publish_button: gtk::Button = self.build_button_with_name(builder, "publish_button");

        let qos_publish_0: gtk::RadioButton =
            self.build_radiobutton_with_name(builder, "qos_publish_0");

        let sender_publish = self.get_clone_sender_of_client();

        publish_button.connect_clicked(move |_| {
            let message = message_input.text().to_string();
            let topic = topic_input.text().to_string();
            let is_qos_0 = qos_publish_0.is_active();

            let publish = Publish::init(message, topic, is_qos_0);
            sender_publish
                .send(InterfaceSender::Publish(publish))
                .unwrap();
        });
    }

    fn get_clone_sender_of_client(&self) -> Sender<InterfaceSender> {
        self.sender_of_client.clone()
    }

}
