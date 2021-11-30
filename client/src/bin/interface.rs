extern crate gtk;
use client::client_for_interface::Client;
use client::client_for_interface::ClientAction;
use client::client_for_interface::SenderClient;
use gtk::glib;
use gtk::prelude::*;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
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
    let result_label: gtk::Label = builder.object("result_label").unwrap();
    let user_input: gtk::Entry = builder.object("user_input").unwrap();
    let id_input: gtk::Entry = builder.object("id_input").unwrap();
    let password_input: gtk::Entry = builder.object("password_input").unwrap();

    (
        input_port,
        ip_input,
        user_input,
        password_input,
        id_input,
        result_label,
        connect_button,
    )
}

fn build_objects_for_publish(builder: &gtk::Builder) -> (gtk::Entry, gtk::Entry, gtk::Button) {
    let message_input: gtk::Entry = builder.object("port_input").unwrap();
    let topic_input: gtk::Entry = builder.object("ip_input").unwrap();
    let publish_message: gtk::Button = builder.object("publish_button").unwrap();
    (message_input, topic_input, publish_message)
}

fn build_ui_for_client(app: &gtk::Application, client_sender: Sender<SenderClient>) {
    let sender_connect = client_sender.clone();
    let sender_publish = client_sender.clone();
    let glade_src = include_str!("test.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.object("main_window").unwrap();
    window.set_application(Some(app));
    let (input_port, ip_input, user_input, password_input, id_input, result_label, connect_button) =
        build_objects_for_connect(&builder);
    let (message_input, topic_input, publish_button) = build_objects_for_publish(&builder);
    let (tx, rc) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    connect_button.connect_clicked(move |_| {
        let port = input_port.text().to_string();
        let ip = ip_input.text().to_string();
        let tx = tx.clone();
        let user = user_input.text().to_string();
        let password = password_input.text().to_string();
        let id_client = id_input.text().to_string();

        let (send_x, rec_x): (Sender<String>, Receiver<String>) = mpsc::channel();
        sender_connect
            .send((
                ClientAction::Connect,
                Some(ip),
                Some(port),
                Some(user),
                Some(password),
                Some(id_client),
                send_x,
            ))
            .unwrap();

        let result = rec_x.recv().unwrap();
        tx.send(result).unwrap();
    });

    publish_button.connect_clicked(move |_| {
        let message = message_input.text().to_string();
        let topic = topic_input.text().to_string();
        let (send_x, rec_x): (Sender<String>, Receiver<String>) = mpsc::channel();
        sender_publish
            .send((
                ClientAction::Publish,
                Some(message),
                Some(topic),
                None,
                None,
                None,
                send_x,
            ))
            .unwrap();
    });

    rc.attach(None, move |text| {
        result_label.set_text(&text);
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
