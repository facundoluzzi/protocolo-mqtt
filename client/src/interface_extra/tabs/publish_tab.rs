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

    /// Crea el comportamiento que va a tener el boton de publicar dentro de la interfaz en la pestania de publicacion
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

    /// Construye todos los elementos que conforman la pestania de publicacion los crea para poder mostrarlos y ademas
    /// les asigna su comportamiento segun corresponda
    pub fn build(&self, builder: &gtk::Builder) {
        self.attach_action_for_publish_button(
            build_button_with_name(builder, "publish_button"),
            build_entry_with_name(builder, "message_input"),
            build_entry_with_name(builder, "topic_input"),
            build_radiobutton_with_name(builder, "qos_publish_0"),
            self.get_clone_sender_of_client(),
        );
    }

    /// Crea un clon del sender que sirve para mandarle al cliente el paquete que debe ser enviado hacia el broker
    fn get_clone_sender_of_client(&self) -> Sender<InterfaceSender> {
        self.sender_of_client.clone()
    }
}
