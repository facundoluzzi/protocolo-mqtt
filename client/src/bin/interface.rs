extern crate gtk;
use client::client_for_interface::Client;
use client::packet_manager::ResponsePacket;
use client::sender_types::connect::Connect;
use client::sender_types::publish::Publish;
use client::sender_types::subscribe::Subscribe;

use client::sender_types::sender_type::ClientSender;
use client::sender_types::sender_type::InterfaceSender;

use gtk::glib;
use gtk::prelude::*;
use std::sync::mpsc::Sender;

fn build_objects_for_connect(
    builder: &gtk::Builder,
) -> (
    gtk::Entry,
    gtk::Entry,
    gtk::Entry,
    gtk::Entry,
    gtk::Entry,
    gtk::Label,
    gtk::Button,
) {
    let input_port: gtk::Entry = builder.object("port_input").unwrap();
    let ip_input: gtk::Entry = builder.object("ip_input").unwrap();
    let connect_button: gtk::Button = builder.object("connect_button").unwrap();
    let result_for_connect: gtk::Label = builder.object("result_label").unwrap();
    let user_input: gtk::Entry = builder.object("user_input").unwrap();
    let id_input: gtk::Entry = builder.object("id_input").unwrap();
    let password_input: gtk::Entry = builder.object("password_input").unwrap();

    (
        input_port,
        ip_input,
        user_input,
        password_input,
        id_input,
        result_for_connect,
        connect_button,
    )
}

fn build_objects_for_publish(
    builder: &gtk::Builder,
) -> (
    gtk::Entry,
    gtk::Entry,
    gtk::Button,
    gtk::RadioButton,
    gtk::RadioButton,
    gtk::Label,
) {
    let message_input: gtk::Entry = builder.object("message_input").unwrap();
    let topic_input: gtk::Entry = builder.object("topic_input").unwrap();
    let publish_message: gtk::Button = builder.object("publish_button").unwrap();
    let qos_publish_1: gtk::RadioButton = builder.object("qos_publish_1").unwrap();
    let qos_publish_0: gtk::RadioButton = builder.object("qos_publish_0").unwrap();
    let result_label_1: gtk::Label = builder.object("result_label1").unwrap();
    (
        message_input,
        topic_input,
        publish_message,
        qos_publish_1,
        qos_publish_0,
        result_label_1,
    )
}

fn build_objects_for_suscribe(
    builder: &gtk::Builder,
) -> (
    gtk::Entry,
    gtk::Button,
    gtk::RadioButton,
    gtk::RadioButton,
    gtk::Label,
    gtk::Label,
) {
    let input_topic_suscribe: gtk::Entry = builder.object("input_topic_suscribe").unwrap();
    let suscribe_button: gtk::Button = builder.object("suscribe_button").unwrap();
    let qos_suscriber_0: gtk::RadioButton = builder.object("qos_suscriber_0").unwrap();
    let qos_suscriber_1: gtk::RadioButton = builder.object("qos_suscriber_1").unwrap();
    let result_label_2: gtk::Label = builder.object("result_label2").unwrap();
    let messages_received: gtk::Label = builder.object("messages_received").unwrap();
    (
        input_topic_suscribe,
        suscribe_button,
        qos_suscriber_0,
        qos_suscriber_1,
        result_label_2,
        messages_received,
    )
}

fn build_ui_for_client(app: &gtk::Application, client_sender: Sender<InterfaceSender>) {
    let sender_connect = client_sender.clone();
    let sender_publish = client_sender.clone();
    let sender_suscribe = client_sender.clone();

    let glade_src = include_str!("test.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.object("main_window").unwrap();
    window.set_application(Some(app));

    let (
        input_port,
        ip_input,
        user_input,
        password_input,
        id_input,
        result_for_connect,
        connect_button,
    ) = build_objects_for_connect(&builder);

    let (
        message_input,
        topic_input,
        publish_button,
        qos_publish_1,
        qos_publish_0,
        result_for_publish,
    ) = build_objects_for_publish(&builder);

    let (
        input_topic_suscribe,
        suscribe_button,
        qos_suscriber_0,
        qos_suscriber_1,
        result_for_suscribe,
        messages_received,
    ) = build_objects_for_suscribe(&builder);

    let (tx, rc) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let tx_for_connection = tx.clone();

    connect_button.connect_clicked(move |_| {
        let port = input_port.text().to_string();
        let ip = ip_input.text().to_string();
        let user = user_input.text().to_string();
        let password = password_input.text().to_string();
        let id_client = id_input.text().to_string();

        let connection = Connect::init(
            ip,
            port,
            user,
            password,
            id_client,
            tx_for_connection.clone(),
        );
        sender_connect
            .send(InterfaceSender::Connect(connection))
            .unwrap();
    });

    publish_button.connect_clicked(move |_| {
        let message = message_input.text().to_string();
        let topic = topic_input.text().to_string();
        let is_qos_0 = qos_publish_0.is_active();

        let publish = Publish::init(message, topic, is_qos_0);
        sender_publish
            .send(InterfaceSender::Publish(publish))
            .unwrap();
    });

    suscribe_button.connect_clicked(move |_| {
        let topic = input_topic_suscribe.text().to_string();
        let is_qos_0 = qos_suscriber_0.is_active();

        let subscribe = Subscribe::init(topic, is_qos_0);
        sender_suscribe
            .send(InterfaceSender::Subscribe(subscribe))
            .unwrap();
    });

    rc.attach(None, move |client_sender| {
        match client_sender {
            ClientSender::Connack(connack) => {
                let response = connack.get_response();
                result_for_connect.set_text(&response);
            }
            ClientSender::Suback(suback) => {
                let response = suback.get_response();
                result_for_suscribe.set_text(&response);
            }
            ClientSender::Puback(puback) => {
                let response = puback.get_response();
                result_for_publish.set_text(&response);
            }
            ClientSender::Publish(publish) => {
                let response = publish.get_response();
                let topic = publish.get_topic();
                messages_received.set_text(&format!("{} en {}", response, topic));
            }
            ClientSender::Default(_default) => {}
        }
        glib::Continue(true)
    });

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default());

    application.connect_activate(|app| {
        let sender_client = Client::init();
        build_ui_for_client(app, sender_client);
    });

    application.run();
}
