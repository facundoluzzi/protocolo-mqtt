use crate::helper::stream::stream_handler::StreamAction::ReadStream;
use crate::packet::input::connect::Connect;
use crate::packet::input::disconnect::Disconnect;
use crate::packet::input::pingreq::Pingreq;
use crate::packet::input::puback_to_send::PubackToSend;
use crate::packet::input::publish::Publish;
use crate::packet::input::subscribe::Subscribe;
use crate::packet::input::unsubscribe::Unsubscribe;
use crate::packet::packet_manager::PacketManager;
use crate::packet::sender_type::ClientSender;
use crate::packet::sender_type::InterfaceSender;
use crate::types::SenderForServer;
use crate::types::StreamType;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SendError;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

pub struct Client {
    sender_stream: Option<Sender<StreamType>>,
    signal_sender: Option<Sender<bool>>,
}

pub enum ClientAction {
    Connect,
    Publish,
    Subscribe,
}

impl Client {
    /// Inicializa el cliente con un receiver que se queda escuchando por eventos, estos eventos
    /// son los que se producen por alguna accion propia de la interfaz, el evento recibido desde ahi
    /// se matchea para poder procesarlos correctamente y enviar el paquete deseado hacia el StreamHandler.
    /// Devuelve el sender que corresponde a este receiver de eventos, va a ser utilizado en la interfaz para mandarlos.
    pub fn init() -> Sender<InterfaceSender> {
        let (event_sender, event_receiver): (Sender<InterfaceSender>, Receiver<InterfaceSender>) =
            mpsc::channel();

        let mut client = Client {
            sender_stream: None,
            signal_sender: None,
        };

        thread::spawn(move || {
            for event in event_receiver {
                match event {
                    InterfaceSender::Connect(connect) => client.process_connect_event(connect),
                    InterfaceSender::Publish(publish) => client.process_publish_event(publish),
                    InterfaceSender::Subscribe(subscribe) => {
                        client.process_subscribe_event(subscribe)
                    }
                    InterfaceSender::Unsubscribe(unsubscribe) => {
                        client.process_unsubscribe_event(unsubscribe)
                    }
                    InterfaceSender::Disconnect(disconnect) => {
                        client.process_disconnect_event(disconnect)
                    }
                    InterfaceSender::PubackToSend(puback_to_send) => {
                        client.process_puback_to_send_event(puback_to_send)
                    }
                }
            }
        });

        event_sender
    }

    /// Procesa el evento creado en la interfaz que corresponde a mandar el puback como respuesta
    fn process_puback_to_send_event(&mut self, puback_to_send: PubackToSend) {
        match self.sender_stream.clone() {
            Some(sender_stream) => {
                if puback_to_send.send_puback(sender_stream).is_err() {
                    println!("Error sending PUBACK")
                };
            }
            None => {
                println!("Unexpected error");
            }
        }
    }

    /// Procesa el evento creado en la interfaz que corresponde al Disconnect
    fn process_disconnect_event(&mut self, disconnect: Disconnect) {
        match self.sender_stream.clone() {
            Some(sender_stream) => match disconnect.send_disconnect(sender_stream) {
                Ok(_result_ok) => {
                    if let Some(signal_sender) = self.signal_sender.clone() {
                        if let Err(err) = signal_sender.send(false) {
                            println!("{}", err);
                        } else {
                            self.signal_sender = None;
                        }
                    }
                }
                Err(err) => {
                    println!("err: {}", err);
                }
            },
            None => {
                println!("Unexpected error");
            }
        }
    }

    /// Procesa el evento creado en la interfaz que corresponde al Unsubscribe
    fn process_unsubscribe_event(&mut self, unsubscribe: Unsubscribe) {
        match self.sender_stream.clone() {
            Some(sender_stream) => match unsubscribe.send_unsubscribe(sender_stream) {
                Ok(_result_ok) => {}
                Err(err) => {
                    println!("err: {}", err);
                }
            },
            None => {
                println!("Unexpected error")
            }
        }
    }

    /// Procesa el evento creado en la interfaz que corresponde al Subscribe
    fn process_subscribe_event(&mut self, subscribe: Subscribe) {
        match self.sender_stream.clone() {
            Some(sender_stream) => match subscribe.send_suscribe(sender_stream) {
                Ok(_result_ok) => {}
                Err(err) => {
                    println!("err: {}", err);
                }
            },
            None => {
                println!("Unexpected error")
            }
        }
    }

    /// Procesa el evento creado en la interfaz que corresponde al Publish
    fn process_publish_event(&mut self, publish: Publish) {
        match self.sender_stream.clone() {
            Some(sender_stream) => match publish.send_publish(sender_stream) {
                Ok(_result_ok) => {}
                Err(err) => {
                    println!("err: {}", err);
                }
            },
            None => {
                println!("Unexpected error");
            }
        }
    }

    /// Procesa el evento creado en la interfaz que corresponde al connect
    fn process_connect_event(&mut self, connect: Connect) {
        let sender_stream = connect.connect_to_server();
        if let Ok(sender) = sender_stream {
            self.sender_stream = Some(sender.clone());

            let (sender_for_ping, receiver_for_ping) = mpsc::channel::<bool>();

            if !connect.keep_alive_is_empty() {
                self.signal_sender = Some(sender_for_ping);
                self.start_to_send_pingreq(&connect, sender.clone(), receiver_for_ping);
            }
            Client::start_to_read(sender, connect.get_gtk_sender());
        }
    }

    /// Procesa un paquete mediante el packet manager para luego enviar el tipo de paquete a la interfaz, para imprimir la respuesta
    /// y procesarlo en ese lugar
    fn process_packet(
        bytes: &[u8],
        sender: gtk::glib::Sender<ClientSender>,
        sender_stream: Sender<StreamType>,
    ) -> Result<(), SendError<ClientSender>> {
        let packet_manager = PacketManager::new();
        let response = packet_manager.process_message(bytes, sender_stream);

        if let Some(client_sender) = response {
            sender.send(client_sender)?;
        };

        Ok(())
    }

    /// Chequea si un paquete (vector de u8) esta vacio
    fn is_empty_packet(packet: Vec<u8>) -> bool {
        !packet.into_iter().any(|element| element != 0)
    }

    /// Comienza a escuchar por paquetes desde el broker, se queda esperando por dichos paquetes y los manda para procesar
    /// una vez que se recibe uno
    fn start_to_read(
        sender_stream: Sender<StreamType>,
        sender_gtk: gtk::glib::Sender<ClientSender>,
    ) {
        thread::spawn(move || {
            let (packet_sender, packet_receiver) = mpsc::channel::<Vec<u8>>();

            loop {
                let message_sent =
                    sender_stream
                        .clone()
                        .send((ReadStream, None, Some(packet_sender.clone())));

                if let Err(_msg) = message_sent {
                } else if let Ok(packet) = packet_receiver.recv() {
                    let empty_packet = Client::is_empty_packet(packet.clone());
                    if empty_packet {
                        break;
                    }
                    let packet_u8: &[u8] = &packet;
                    if let Err(err) =
                        Client::process_packet(packet_u8, sender_gtk.clone(), sender_stream.clone())
                    {
                        println!("err: {}", err);
                        break;
                    }
                }
            }
        });
    }

    /// Inicia a mandar el pingreq en caso que sea necesario, una vez que recibe la senial a traves del channel
    /// significa que el cliente se desconecto y ya no es necesario enviar el pingreq, asi que deja de hacerlo.
    fn start_to_send_pingreq(
        &self,
        connect: &Connect,
        sender: SenderForServer,
        receiver_for_ping: Receiver<bool>,
    ) {
        let pingreq = Pingreq::init(connect.get_keep_alive());
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(pingreq.get_interval() as u64));
            let keep_sending = receiver_for_ping.try_recv();
            if keep_sending.is_ok() {
                return;
            }
            match pingreq.send_pingreq(sender.clone()) {
                Ok(_result) => {}
                Err(err) => {
                    println!("err: {}", err);
                }
            }
        });
    }
}
