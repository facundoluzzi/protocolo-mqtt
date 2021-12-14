use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};
extern crate gtk;
use gtk::prelude::*;

use crate::interface_extra::object_builder::{
    build_button_with_name, build_entry_with_name, build_label_with_name,
    build_radiobutton_with_name,
};

use crate::packet::{
    input::{subscribe::Subscribe, unsubscribe::Unsubscribe},
    sender_type::InterfaceSender,
};

type SenderNewTopicsAndQoS = Sender<(String, bool)>;
type ReceiverNewTopicsAndQoS = Receiver<(String, bool)>;

pub struct SubscribeTab {
    sender_of_client: Sender<InterfaceSender>,
}

impl SubscribeTab {
    pub fn new(sender_of_client: Sender<InterfaceSender>) -> Self {
        SubscribeTab { sender_of_client }
    }

    fn attach_action_for_unsubscribe_button(
        &self,
        unsubscribe_button: gtk::Button,
        input_topic_unsubscribe: gtk::Entry,
        cloned_topic_list_label: gtk::Label,
        sender_unsubscribe: Sender<InterfaceSender>,
    ) {
        unsubscribe_button.connect_clicked(move |_| {
            let topic = input_topic_unsubscribe.text().to_string();

            let topic_list = cloned_topic_list_label
                .text()
                .to_string()
                .split('\n')
                .filter(|line| !line.contains(&topic))
                .collect::<Vec<&str>>()
                .join("\n");

            cloned_topic_list_label.set_text(&topic_list);

            let unsubscribe = Unsubscribe::init(topic);
            if let Err(_err) = sender_unsubscribe.send(InterfaceSender::Unsubscribe(unsubscribe)) {
                println!("Error desuscribiendose");
            }
        });
    }

    fn attach_action_for_suscribe_button(
        &self,
        subscribe_button: gtk::Button,
        data_for_thread_dos: Arc<Mutex<Vec<(String, bool)>>>,
        sender_subscribe: Sender<InterfaceSender>,
    ) {
        subscribe_button.connect_clicked(move |_| {
            if let Ok(mut data) = data_for_thread_dos.lock() {
                let subscribe = Subscribe::init(data.to_vec());
                data.clear();
                if let Err(_err) = sender_subscribe.send(InterfaceSender::Subscribe(subscribe)) {
                    println!("Error en la suscripcion");
                }
            };
        });
    }

    fn attach_action_for_add_topic_button(
        &self,
        add_topic_button: gtk::Button,
        input_topic_subscribe: gtk::Entry,
        qos_subscriber_0: gtk::RadioButton,
        sender_for_new_topics: gtk::glib::Sender<(String, bool)>,
    ) {
        add_topic_button.connect_clicked(move |_| {
            let topic = input_topic_subscribe.text().to_string();
            let is_qos_0 = qos_subscriber_0.is_active();
            let sender = sender_for_new_topics.clone();
            if let Err(_err) = sender.send((topic, is_qos_0)) {
                println!("Error mostrando nuevo topic");
            }
            input_topic_subscribe.set_text("");
        });
    }

    pub fn build(&self, builder: &gtk::Builder) {
        let input_topic_subscribe: gtk::Entry =
            build_entry_with_name(builder, "input_topic_subscribe");
        let subscribe_button: gtk::Button = build_button_with_name(builder, "subscribe_button");
        let qos_subscriber_0: gtk::RadioButton =
            build_radiobutton_with_name(builder, "qos_subscriber_0");
        let unsubscribe_button: gtk::Button = build_button_with_name(builder, "unsubscribe_button");
        let input_topic_unsubscribe: gtk::Entry =
            build_entry_with_name(builder, "input_topic_unsubscribe");
        let add_topic_button: gtk::Button = build_button_with_name(builder, "add_topic_button");
        let topic_list_label: gtk::Label = build_label_with_name(builder, "topic_list_label");
        let sender_unsubscribe = self.get_clone_sender_of_client();

        let list_of_topics_to_suscribe = Vec::new();
        let data = Arc::new(Mutex::new(list_of_topics_to_suscribe));
        let data_for_thread = data.clone();
        let data_for_thread_dos = data;

        let cloned_topic_list_label = topic_list_label.clone();
        let sender_subscribe = self.get_clone_sender_of_client();

        let (sender_t, receiver_t): (SenderNewTopicsAndQoS, ReceiverNewTopicsAndQoS) =
            mpsc::channel();

        let (sender_for_new_topics, receiver_for_new_topics) =
            glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            for received_topic in receiver_t {
                if let Ok(mut data) = data_for_thread.lock() {
                    data.push(received_topic);
                };
            }
        });

        self.attach_action_for_unsubscribe_button(
            unsubscribe_button,
            input_topic_unsubscribe,
            cloned_topic_list_label,
            sender_unsubscribe,
        );
        self.attach_action_for_suscribe_button(
            subscribe_button,
            data_for_thread_dos,
            sender_subscribe,
        );
        self.attach_action_for_add_topic_button(
            add_topic_button,
            input_topic_subscribe,
            qos_subscriber_0,
            sender_for_new_topics,
        );

        receiver_for_new_topics.attach(None, move |(topic, qos)| {
            let actual = topic_list_label.text();
            let cloned_sender_t = sender_t.clone();
            if qos {
                topic_list_label.set_text(&(actual.to_string() + &format!("\n{},{}", topic, 0)));
            } else {
                topic_list_label.set_text(&(actual.to_string() + &format!("\n{},{}", topic, 1)));
            }

            if let Err(_err) = cloned_sender_t.send((topic, qos)) {
                println!("Error mostrando nuevo topic");
            }

            glib::Continue(true)
        });
    }

    fn get_clone_sender_of_client(&self) -> Sender<InterfaceSender> {
        self.sender_of_client.clone()
    }
}
