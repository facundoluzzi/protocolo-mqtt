extern crate gtk;

use client::client_for_interface::Client;
use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use std::thread;

fn build_objects_for_connect (builder: &gtk::Builder) -> (gtk::Entry, gtk::Entry, gtk::Entry, gtk::Entry,gtk::Entry ,gtk::Label, gtk::Button) {
    let input_port: gtk::Entry = builder.object("port_input").unwrap();
    let ip_input: gtk::Entry = builder.object("ip_input").unwrap();
    let connect_button: gtk::Button = builder.object("connect_button").unwrap();
    let result_label: gtk::Label = builder.object("result_label").unwrap();
    let user_input: gtk::Entry = builder.object("user_input").unwrap();
    let id_input: gtk::Entry = builder.object("id_input").unwrap();
    let password_input: gtk::Entry = builder.object("password_input").unwrap();

    (input_port, ip_input, user_input, password_input, id_input, result_label, connect_button)
}

fn build_objects_for_publish (builder: &gtk::Builder) -> (gtk::Entry, gtk::Entry, gtk::Button) {
    let message_input: gtk::Entry = builder.object("port_input").unwrap();
    let topic_input: gtk::Entry = builder.object("ip_input").unwrap();
    let publish_message: gtk::Button = builder.object("connect_button").unwrap();
    (message_input, topic_input, publish_message)
}

fn build_ui_for_client(app: &gtk::Application, client: Client) {
    let glade_src = include_str!("test.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.object("main_window").unwrap();
    window.set_application(Some(app));
    let (input_port, ip_input, user_input, password_input, id_input, result_label, connect_button) = build_objects_for_connect(&builder);
    let (message_input, topic_input, publish_message) = build_objects_for_publish(&builder);
    let (tx, rc) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    connect_button.connect_clicked(clone!(@weak input_port, @weak ip_input, @weak password_input, @weak user_input => move |_| {
        let port = input_port.text().to_string();
        let ip = ip_input.text().to_string();
        let tx = tx.clone();
        let user = user_input.text().to_string();
        let password = password_input.text().to_string();
        let id_client = id_input.text().to_string();
        thread::spawn(move || {
            let client = Client::new();
            let result = client.connect_to_server(ip, port, user, password, id_client);
            tx.send(result).unwrap();
        });
    }));

    publish_message.connect_clicked(clone!(@weak message_input, @weak topic_input => move |_| {
        let message = message_input.text().to_string();
        let topic = topic_input.text().to_string();
        thread::spawn(move || {
            
        });
    }));

    rc.attach(None, move |text| {
        match text {
            Ok(label) => {
                result_label.set_text(&label);
            }
            Err(label) => {
                result_label.set_text(&label);
            }
        }

        glib::Continue(true)
    });

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default());

    application.connect_activate(|app| {
        let client = Client::new();
        build_ui_for_client(app, client);
    });

    application.run();
}
