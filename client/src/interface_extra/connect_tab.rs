use std::sync::mpsc::Sender;
extern crate gtk;
use gtk::prelude::*;

use crate::packet::input::connect::Connect;
use crate::packet::input::disconnect::Disconnect;
use crate::packet::output::connect_error_response::ConnectErrorResponse;
use crate::packet::output::disconnect_response::DisconnectResponse;
use crate::packet::sender_type::{ClientSender, InterfaceSender};

pub struct ConnectTab {
    sender_of_client: Sender<InterfaceSender>,
    sender_for_client: gtk::glib::Sender<ClientSender>,
}

impl ConnectTab {
    pub fn new(
        sender_of_client: Sender<InterfaceSender>,
        sender_for_client: gtk::glib::Sender<ClientSender>,
    ) -> Self {
        ConnectTab {
            sender_of_client,
            sender_for_client,
        }
    }

    pub fn build(&self, builder: &gtk::Builder) {
        let input_port: gtk::Entry = self.build_entry_with_name(builder, "port_input");
        let ip_input: gtk::Entry = self.build_entry_with_name(builder, "ip_input");
        let user_input: gtk::Entry = self.build_entry_with_name(builder, "user_input");
        let id_input: gtk::Entry = self.build_entry_with_name(builder, "id_input");
        let password_input: gtk::Entry = self.build_entry_with_name(builder, "password_input");
        let last_will_message_input: gtk::Entry =
            self.build_entry_with_name(builder, "last_will_message_input");
        let last_will_topic_input: gtk::Entry =
            self.build_entry_with_name(builder, "last_will_topic_input");
        let keep_alive_input: gtk::Entry = self.build_entry_with_name(builder, "keep_alive_input");

        let connect_button: gtk::Button = self.build_button_with_name(builder, "connect_button");
        let disconnect_button: gtk::Button =
            self.build_button_with_name(builder, "disconnect_button");

        let clean_session_checkbox: gtk::CheckButton =
            self.build_checkbutton_with_name(builder, "clean_session_checkbox");
        let qos_will_message_0: gtk::RadioButton =
            self.build_radiobutton_with_name(builder, "qos_will_message_0");

        let sender_connect = self.get_clone_sender_of_client();
        let sender_disconnect = self.get_clone_sender_of_client();

        let tx_for_error_connection = self.get_clone_sender_for_client();
        let tx_for_connection = self.get_clone_sender_for_client();
        let tx_for_disconnection = self.get_clone_sender_for_client();

        disconnect_button.connect_clicked(move |_| {
            let disconnect = Disconnect::init();

            sender_disconnect
                .send(InterfaceSender::Disconnect(disconnect))
                .unwrap();

            let disconnect_response = DisconnectResponse::init();
            if let Err(err) =
                tx_for_disconnection.send(ClientSender::Disconnect(disconnect_response))
            {
                println!("err: {}", err);
            }
        });

        connect_button.connect_clicked(move |_| {
            let port = input_port.text().to_string();
            let ip = ip_input.text().to_string();
            let user = user_input.text().to_string();
            let password = password_input.text().to_string();
            let id_client = id_input.text().to_string();
            let last_will_message = last_will_message_input.text().to_string();
            let last_will_topic = last_will_topic_input.text().to_string();
            let clean_session_is_active = clean_session_checkbox.is_active();
            let qos_will_message_is_0 = qos_will_message_0.is_active();
            let keep_alive = keep_alive_input.text().to_string();

            if id_client.is_empty() && !clean_session_is_active {
                let connect_error = ConnectErrorResponse::init(
                    "ClientID requerido o activar Clean Session".to_string(),
                );
                tx_for_error_connection
                    .send(ClientSender::ConnectError(connect_error))
                    .unwrap();
                return;
            }

            let connection = Connect::init(
                ip,
                port,
                user,
                password,
                id_client,
                tx_for_connection.clone(),
                last_will_message,
                last_will_topic,
                clean_session_is_active,
                qos_will_message_is_0,
                keep_alive,
            );

            sender_connect
                .send(InterfaceSender::Connect(connection))
                .unwrap();
            println!("4");
        });
    }

    fn get_clone_sender_of_client(&self) -> Sender<InterfaceSender> {
        self.sender_of_client.clone()
    }

    fn get_clone_sender_for_client(&self) -> gtk::glib::Sender<ClientSender> {
        self.sender_for_client.clone()
    }
}
