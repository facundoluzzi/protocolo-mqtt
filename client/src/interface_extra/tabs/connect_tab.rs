use std::sync::mpsc::Sender;
extern crate gtk;
use crate::packet::input::connect::Connect;
use crate::packet::input::disconnect::Disconnect;
use crate::packet::output::connect_error_response::ConnectErrorResponse;
use crate::packet::output::disconnect_response::DisconnectResponse;
use crate::packet::sender_type::{ClientSender, InterfaceSender};
use gtk::prelude::*;

use crate::interface_extra::object_builder::{
    build_button_with_name, build_checkbutton_with_name, build_entry_with_name,
    build_radiobutton_with_name,
};

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

    fn attach_action_for_disconnect_button(
        &self,
        disconnect_button: gtk::Button,
        sender_disconnect: Sender<InterfaceSender>,
        tx_for_disconnection: gtk::glib::Sender<ClientSender>,
    ) {
        disconnect_button.connect_clicked(move |_| {
            let disconnect = Disconnect::init();
            if let Err(_error) = sender_disconnect.send(InterfaceSender::Disconnect(disconnect)) {
                println!("Error en el disconnect");
                return;
            }
            let disconnect_response = DisconnectResponse::init();
            if let Err(err) =
                tx_for_disconnection.send(ClientSender::Disconnect(disconnect_response))
            {
                println!("err: {}", err);
            }
        });
    }

    fn attach_action_for_connect_button(
        &self,
        connect_button: gtk::Button,
        entry_vec: Vec<gtk::Entry>,
        clean_session_checkbox: gtk::CheckButton,
        qos_will_message_0: gtk::RadioButton,
        tx_for_connection: gtk::glib::Sender<ClientSender>,
        sender_connect: Sender<InterfaceSender>,
    ) {
        connect_button.connect_clicked(move |_| {
            let port = entry_vec[0].text().to_string();
            let ip = entry_vec[1].text().to_string();
            let user = entry_vec[2].text().to_string();
            let password = entry_vec[3].text().to_string();
            let id_client = entry_vec[4].text().to_string();
            let last_will_message = entry_vec[5].text().to_string();
            let last_will_topic = entry_vec[6].text().to_string();
            let clean_session_is_active = clean_session_checkbox.is_active();
            let qos_will_message_is_0 = qos_will_message_0.is_active();
            let keep_alive = entry_vec[7].text().to_string();
            let tx_for_error_connection = tx_for_connection.clone();

            if id_client.is_empty() && !clean_session_is_active {
                let connect_error = ConnectErrorResponse::init(
                    "ClientID requerido o activar Clean Session".to_string(),
                );
                if let Ok(()) =
                    tx_for_error_connection.send(ClientSender::ConnectError(connect_error))
                {
                    return;
                }
            }

            let connection = Connect::init(
                vec![
                    ip,
                    port,
                    user,
                    password,
                    id_client,
                    last_will_message,
                    last_will_topic,
                ],
                tx_for_connection.clone(),
                clean_session_is_active,
                qos_will_message_is_0,
                keep_alive,
            );

            if let Err(_error) = sender_connect.send(InterfaceSender::Connect(connection)) {
                println!("Error en el connect");
            }
        });
    }

    pub fn build(&self, builder: &gtk::Builder) {
        let input_port: gtk::Entry = build_entry_with_name(builder, "port_input");
        let ip_input: gtk::Entry = build_entry_with_name(builder, "ip_input");
        let user_input: gtk::Entry = build_entry_with_name(builder, "user_input");
        let id_input: gtk::Entry = build_entry_with_name(builder, "id_input");
        let password_input: gtk::Entry = build_entry_with_name(builder, "password_input");
        let last_will_message_input: gtk::Entry =
            build_entry_with_name(builder, "last_will_message_input");
        let last_will_topic_input: gtk::Entry =
            build_entry_with_name(builder, "last_will_topic_input");
        let keep_alive_input: gtk::Entry = build_entry_with_name(builder, "keep_alive_input");

        let connect_button: gtk::Button = build_button_with_name(builder, "connect_button");
        let disconnect_button: gtk::Button = build_button_with_name(builder, "disconnect_button");

        let clean_session_checkbox: gtk::CheckButton =
            build_checkbutton_with_name(builder, "clean_session_checkbox");
        let qos_will_message_0: gtk::RadioButton =
            build_radiobutton_with_name(builder, "qos_will_message_0");

        let sender_connect = self.get_clone_sender_of_client();
        let sender_disconnect = self.get_clone_sender_of_client();

        let tx_for_connection = self.get_clone_sender_for_client();
        let tx_for_disconnection = self.get_clone_sender_for_client();

        self.attach_action_for_disconnect_button(
            disconnect_button,
            sender_disconnect,
            tx_for_disconnection,
        );
        self.attach_action_for_connect_button(
            connect_button,
            vec![
                input_port,
                ip_input,
                user_input,
                password_input,
                id_input,
                last_will_message_input,
                last_will_topic_input,
                keep_alive_input,
            ],
            clean_session_checkbox,
            qos_will_message_0,
            tx_for_connection,
            sender_connect,
        );
    }

    fn get_clone_sender_of_client(&self) -> Sender<InterfaceSender> {
        self.sender_of_client.clone()
    }

    fn get_clone_sender_for_client(&self) -> gtk::glib::Sender<ClientSender> {
        self.sender_for_client.clone()
    }
}
