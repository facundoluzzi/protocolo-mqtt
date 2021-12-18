extern crate gtk;
use client::client_for_interface::Client;
use client::packet::input::puback_to_send::PubackToSend;
use client::packet::sender_type::ClientSender;
use client::packet::sender_type::InterfaceSender;
use std::str::from_utf8;

use gtk::glib;
use gtk::prelude::*;

use std::sync::mpsc::Sender;

use client::interface_extra::object_builder::build_label_with_name;
use client::interface_extra::tabs::connect_tab::ConnectTab;
use client::interface_extra::tabs::publish_tab::PublishTab;
use client::interface_extra::tabs::subscribe_tab::SubscribeTab;

struct AppUI {
    connect_tab: ConnectTab,
    subscribe_tab: SubscribeTab,
    publish_tab: PublishTab,
}

impl AppUI {
    fn new(connect_tab: ConnectTab, subscribe_tab: SubscribeTab, publish_tab: PublishTab) -> Self {
        AppUI {
            connect_tab,
            subscribe_tab,
            publish_tab,
        }
    }

    pub fn start_receiving_responses(
        &self,
        rc: gtk::glib::Receiver<ClientSender>,
        result_for_connect: gtk::Label,
        result_for_publish: gtk::Label,
        result_suback_unsuback: gtk::Label,
        messages_received_label: gtk::Label,
        sender_stream: Sender<InterfaceSender>,
    ) {
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
                    let mut message = publish.get_response();
                    let topic = publish.get_topic();
                    message.push_str(" en topic ");
                    message.push_str(&topic);
                    let result = match from_utf8(message.as_bytes()) {
                        Ok(result) => result,
                        Err(_err) => "There's been a problem getting the text",
                    };
                    messages_received_label
                        .set_text(&(messages_received_label.text().to_string() + result + "\n"));

                    if publish.get_qos() == 0x01 {
                        let puback = PubackToSend::init(publish.get_packet_identifier());
                        if let Err(err) = sender_stream.send(InterfaceSender::PubackToSend(puback)) {
                            println!("Error mandando puback al broker {}", err);
                        }
                    }
                }
                ClientSender::Unsuback(unsuback) => {
                    let response = unsuback.get_response();
                    result_suback_unsuback.set_text(&response);
                }
                ClientSender::Disconnect(disconnect) => {
                    let response = disconnect.get_response();
                    result_for_connect.set_text(&response);
                }
                ClientSender::ConnectError(connect) => {
                    let error_response = connect.get_response();
                    result_for_connect.set_text(&error_response);
                }
                ClientSender::Pingresp(_pingresp) => {}
                ClientSender::Default(_default) => {}
            }
            glib::Continue(true)
        });
    }
}

fn build_ui_for_client(app: &gtk::Application, client_sender: Sender<InterfaceSender>) {
    let glade_src = include_str!("test.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = match builder.object("main_window") {
        Some(window) => window,
        None => panic!("Can not create the UI"),
    };

    let (tx_for_connection, rc) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let result_for_connect: gtk::Label = build_label_with_name(&builder, "result_label");
    let result_label_1: gtk::Label = build_label_with_name(&builder, "result_label1");
    let result_label_2: gtk::Label = build_label_with_name(&builder, "result_suback_unsubcak");
    let messages_received_label: gtk::Label =
        build_label_with_name(&builder, "messages_received_label");

    let connect_tab = ConnectTab::new(client_sender.clone(), tx_for_connection);
    let subscribe_tab = SubscribeTab::new(client_sender.clone());
    let publish_tab = PublishTab::new(client_sender.clone());

    let app_window = AppUI::new(connect_tab, subscribe_tab, publish_tab);
    app_window.connect_tab.build(&builder);
    app_window.subscribe_tab.build(&builder);
    app_window.publish_tab.build(&builder);
    app_window.start_receiving_responses(
        rc,
        result_for_connect,
        result_label_1,
        result_label_2,
        messages_received_label,
        client_sender,
    );

    window.set_application(Some(app));
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
