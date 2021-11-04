extern crate glib;
extern crate gtk;

use glib::clone;
use gtk::prelude::*;

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("test.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.object("main_window").unwrap();
    window.set_application(Some(app));
    let input_port: gtk::Entry = builder.object("port_input").unwrap();
    let button_port: gtk::Button = builder.object("port_button").unwrap();

    button_port.connect_clicked(clone!(@weak input_port => move |_| {
        let port = input_port.text();
        println!("{}", port);

    }));

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default());

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run();
}
