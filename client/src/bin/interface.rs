extern crate gtk;
use client::client_for_interface::Client;
use client::packet::input::unsubscribe::Unsubscribe;
use client::packet::input::connect::Connect;
use client::packet::input::publish::Publish;
use client::packet::input::subscribe::Subscribe;
use std::str::from_utf8;

use client::packet::sender_type::ClientSender;
use client::packet::sender_type::InterfaceSender;

use gtk::glib;
use gtk::prelude::*;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

type SenderNewTopicsAndQoS = Sender<(String, bool)>;
type ReceiverNewTopicsAndQoS = Receiver<(String, bool)>;

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
    gtk::Label,
) {
    let message_input: gtk::Entry = builder.object("message_input").unwrap();
    let topic_input: gtk::Entry = builder.object("topic_input").unwrap();
    let publish_message: gtk::Button = builder.object("publish_button").unwrap();
    let qos_publish_0: gtk::RadioButton = builder.object("qos_publish_0").unwrap();
    let result_label_1: gtk::Label = builder.object("result_label1").unwrap();
    (
        message_input,
        topic_input,
        publish_message,
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
    gtk::Label,
    gtk::Button,
    gtk::Entry,
    gtk::Button,
    gtk::Label,
    gtk::Label,
) {
    let input_topic_suscribe: gtk::Entry = builder.object("input_topic_suscribe").unwrap();
    let suscribe_button: gtk::Button = builder.object("suscribe_button").unwrap();
    let qos_suscriber_0: gtk::RadioButton = builder.object("qos_suscriber_0").unwrap();
    let result_label_2: gtk::Label = builder.object("result_label2").unwrap();
    let unsubscribe_button: gtk::Button = builder.object("suscribe_button").unwrap();
    let input_topic_unsubscribe: gtk::Entry = builder.object("input_topic_unsubscribe").unwrap();
    let add_topic_button: gtk::Button = builder.object("add_button").unwrap();
    let topic_list_label: gtk::Label = builder.object("topic_list_label").unwrap();
    let messages_received_label: gtk::Label = builder.object("messages_received_label").unwrap();
    (
        input_topic_suscribe,
        add_topic_button,
        qos_suscriber_0,
        result_label_2,
        suscribe_button,
        input_topic_unsubscribe,
        unsubscribe_button,
        topic_list_label,
        messages_received_label,
    )
}

fn build_ui_for_client(app: &gtk::Application, client_sender: Sender<InterfaceSender>) {
    let sender_connect = client_sender.clone();
    let sender_publish = client_sender.clone();
    let sender_subscribe = client_sender.clone();
    let sender_unsubscribe = client_sender.clone();

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

    let (message_input, topic_input, publish_button, qos_publish_0, result_for_publish) =
        build_objects_for_publish(&builder);

    let (
        input_topic_suscribe,
        add_topic_button,
        qos_suscriber_0,
        result_suback_unsuback,
        suscribe_button,
        input_topic_unsubscribe,
        unsubscribe_button,
        topic_list_label,
        messages_received_label,
    ) = build_objects_for_suscribe(&builder);

    let (tx_for_connection, rc) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let (sender_for_new_topics, receiver_for_new_topics) =
        glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let list_of_topics_to_suscribe = Vec::new();
    let data = Arc::new(Mutex::new(list_of_topics_to_suscribe));
    let data_for_thread = data.clone();
    let data_for_thread_dos = data;

    let (sender_t, receiver_t): (SenderNewTopicsAndQoS, ReceiverNewTopicsAndQoS) = mpsc::channel();

    thread::spawn(move || {
        for received_topic in receiver_t {
            let mut data = data_for_thread.lock().unwrap();
            data.push(received_topic);
        }
    });

    add_topic_button.connect_clicked(move |_| {
        let topic = input_topic_suscribe.text().to_string();
        let is_qos_0 = qos_suscriber_0.is_active();
        let sender = sender_for_new_topics.clone();
        sender.send((topic, is_qos_0)).unwrap();
        input_topic_suscribe.set_text("");
    });

    receiver_for_new_topics.attach(None, move |(topic, qos)| {
        let actual = topic_list_label.text();
        let cloned_sender_t = sender_t.clone();
        if qos {
            topic_list_label.set_text(&(actual.to_string() + &format!("\n{},{}", topic, 0)));
        } else {
            topic_list_label.set_text(&(actual.to_string() + &format!("\n{},{}", topic, 1)));
        }

        cloned_sender_t.send((topic, qos)).unwrap();

        glib::Continue(true)
    });

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

        println!("\n\n\n\n");
        println!("Message: {}", message);
        println!("Topic: {}", topic);
        println!("is_qos_0: {}", is_qos_0);

        let publish = Publish::init(message, topic, is_qos_0);
        sender_publish
            .send(InterfaceSender::Publish(publish))
            .unwrap();
    });

    suscribe_button.connect_clicked(move |_| {
        // let list_of_topics_to_suscribe_cloned = list_of_topics_to_suscribe.clone();
        let mut data = data_for_thread_dos.lock().unwrap();
        let subscribe = Subscribe::init(data.to_vec());
        data.clear();
        sender_subscribe
            .send(InterfaceSender::Subscribe(subscribe))
            .unwrap();
    });

    unsubscribe_button.connect_clicked(move |_| {
        let topic = input_topic_unsubscribe.text().to_string();

        let unsubscribe = Unsubscribe::init(topic);
        sender_unsubscribe
            .send(InterfaceSender::Unsubscribe(unsubscribe))
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
                result_suback_unsuback.set_text(&response);
            }
            ClientSender::Puback(puback) => {
                let response = puback.get_response();
                result_for_publish.set_text(&response);
            }
            ClientSender::Publish(publish) => {
                println!("ENTRO a mostrar el publish");
                let mut message = publish.get_response();
                let topic = publish.get_topic();
                message.push_str(" en topic ");
                message.push_str(&topic);
                let result = from_utf8(message.as_bytes()).unwrap();
                messages_received_label
                    .set_text(&(messages_received_label.text().to_string() + result + "\n"));
                println!("TERMINO de mostrar el publish");
            }
            ClientSender::Unsuback(unsuback) => {
                let response = unsuback.get_response();
                result_suback_unsuback.set_text(&response);
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
