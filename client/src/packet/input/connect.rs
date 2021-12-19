use crate::helper::stream::stream_handler::Stream;
use crate::helper::stream::stream_handler::StreamAction::WriteStream;
use crate::packet::sender_type::ClientSender;
use crate::types::StreamType;
use std::net::TcpStream;
use std::sync::mpsc::Sender;

pub struct Connect {
    list_of_inputs: Vec<String>,
    send_x: gtk::glib::Sender<ClientSender>,
    clean_session_is_active: bool,
    qos_will_message_0: bool,
    keep_alive: String,
}

impl Connect {
    pub fn init(
        list_of_inputs: Vec<String>,
        send_x: gtk::glib::Sender<ClientSender>,
        clean_session_is_active: bool,
        qos_will_message_0: bool,
        keep_alive: String,
    ) -> Connect {
        Connect {
            list_of_inputs,
            send_x,
            clean_session_is_active,
            qos_will_message_0,
            keep_alive,
        }
    }

    pub fn connect_to_server(&self) -> Result<Sender<StreamType>, String> {
        let address = format!("{}:{}", self.list_of_inputs[0], self.list_of_inputs[1]);
        match TcpStream::connect(address) {
            Ok(stream) => {
                if let Ok(sender_stream) = Stream::init(stream) {
                    self.send_connect(sender_stream.clone())?;
                    Ok(sender_stream)
                } else {
                    Err("Error clonando inicializando el stream".to_string())
                }
            }
            Err(err) => {
                println!("Failed to connect: {}", err);
                let response = "La conexion no se ha podido establecer".to_string();
                Err(response)
            }
        }
    }

    pub fn get_gtk_sender(&self) -> gtk::glib::Sender<ClientSender> {
        self.send_x.clone()
    }

    pub fn keep_alive_is_empty(&self) -> bool {
        self.keep_alive.is_empty() || self.get_keep_alive() == 0
    }

    pub fn get_keep_alive(&self) -> i32 {
        match self.keep_alive.parse::<i32>() {
            Ok(keep_alive) => keep_alive,
            Err(_err) => 0,
        }
    }

    fn send_connect(&self, sender_stream: Sender<StreamType>) -> Result<(), String> {
        let connect_bytes = self.build_bytes_for_connect();
        if sender_stream
            .send((WriteStream, Some(connect_bytes), None))
            .is_err()
        {
            return Err("Error enviando el paquete connect".to_string());
        }

        Ok(())
    }

    fn add_client_id_bytes(&self, flags: &mut u8, bytes: &mut Vec<u8>) {
        if !self.list_of_inputs[4].is_empty() {
            let id_length = self.list_of_inputs[4].len();
            let mut id_client_in_bytes = self.list_of_inputs[4].as_bytes().to_vec();
            bytes.push(0x00);
            bytes.push(id_length as u8);
            bytes.append(&mut id_client_in_bytes);
        }

        if self.clean_session_is_active {
            *flags |= 0b00000010;
        }
    }

    fn add_password_bytes(&self, flags: &mut u8, bytes: &mut Vec<u8>) {
        if !self.list_of_inputs[3].is_empty() {
            *flags |= 0b01000000;
            let password_length = self.list_of_inputs[3].len();
            let mut password_in_bytes = self.list_of_inputs[3].as_bytes().to_vec();
            bytes.push(0x00);
            bytes.push(password_length as u8);
            bytes.append(&mut password_in_bytes);
        }
    }

    fn add_username_bytes(&self, flags: &mut u8, bytes: &mut Vec<u8>) {
        if !self.list_of_inputs[2].is_empty() {
            *flags |= 0b10000000;
            let user_length = self.list_of_inputs[2].len();
            let mut user_in_bytes = self.list_of_inputs[2].as_bytes().to_vec();
            bytes.push(0x00);
            bytes.push(user_length as u8);
            bytes.append(&mut user_in_bytes);
        }
    }

    fn add_keep_alive_bytes(&self, bytes: &mut Vec<u8>) {
        bytes.push(0x00);
        if !self.keep_alive_is_empty() {
            let keep_alive = self.get_keep_alive();
            let keep_alive_as_u8 = keep_alive as u8;
            bytes.push(keep_alive_as_u8);
        } else {
            bytes.push(0x00);
        }
    }

    fn add_will_topic_bytes(&self, flags: &mut u8, bytes: &mut Vec<u8>) {
        if !self.list_of_inputs[6].is_empty() {
            *flags |= 0b00000100;
            let will_topic_length = self.list_of_inputs[6].len();
            let mut will_topic_in_bytes = self.list_of_inputs[6].as_bytes().to_vec();
            bytes.push(0x00);
            bytes.push(will_topic_length as u8);
            bytes.append(&mut will_topic_in_bytes);
        }
    }

    fn add_will_message_bytes(&self, flags: &mut u8, bytes: &mut Vec<u8>) {
        if !self.list_of_inputs[5].is_empty() {
            *flags |= 0b00000100;
            let will_message_length = self.list_of_inputs[5].len();
            let mut will_message_in_bytes = self.list_of_inputs[5].as_bytes().to_vec();
            bytes.push(0x00);
            bytes.push(will_message_length as u8);
            bytes.append(&mut will_message_in_bytes);
        }
    }

    fn change_flag_for_will_qos(&self, flags: &mut u8) {
        if !self.list_of_inputs[5].is_empty() && !self.qos_will_message_0 {
            *flags |= 0b00010000;
        }
    }

    fn build_bytes_for_connect(&self) -> Vec<u8> {
        let mut flags: u8 = 0x00;
        let mut bytes = vec![
            0x10, //Packet ID
            //0x00, Remaining Length
            0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, // Variable Header
            0x04, // Protocol
            0x00, //Flags
        ];
        self.add_keep_alive_bytes(&mut bytes);
        self.add_client_id_bytes(&mut flags, &mut bytes);
        self.add_will_topic_bytes(&mut flags, &mut bytes);
        self.add_will_message_bytes(&mut flags, &mut bytes);
        self.add_username_bytes(&mut flags, &mut bytes);
        self.add_password_bytes(&mut flags, &mut bytes);
        self.change_flag_for_will_qos(&mut flags);
        bytes[8] = flags;
        let length = bytes.len();
        bytes.insert(1, (length - 1) as u8);
        bytes
    }
}
