use crate::sender_types::sender_type::ClientSender;
use crate::stream::stream_handler::Stream;
use crate::stream::stream_handler::StreamAction::WriteStream;
use crate::stream::stream_handler::StreamType;
use std::net::TcpStream;
use std::sync::mpsc::Sender;

pub struct Connect {
    ip: String,
    port: String,
    user: String,
    password: String,
    id_client: String,
    send_x: gtk::glib::Sender<ClientSender>,
}

impl Connect {
    pub fn init(
        ip: String,
        port: String,
        user: String,
        password: String,
        id_client: String,
        send_x: gtk::glib::Sender<ClientSender>,
    ) -> Connect {
        Connect {
            ip,
            port,
            user,
            password,
            id_client,
            send_x,
        }
    }

    pub fn connect_to_server(&self) -> Result<Sender<StreamType>, String> {
        let address = format!("{}:{}", self.ip, self.port);
        match TcpStream::connect(address) {
            Ok(stream) => {
                let sender_stream = Stream::init(stream);
                self.send_connect(sender_stream.clone());

                Ok(sender_stream)
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

    fn send_connect(&self, sender_stream: Sender<StreamType>) {
        let connect_bytes = self.build_bytes_for_connect();
        sender_stream
            .send((WriteStream, Some(connect_bytes), None))
            .unwrap();
    }

    fn add_client_id_bytes(&self, bytes: &mut Vec<u8>) {
        if !self.id_client.is_empty() {
            let id_length = self.id_client.len();
            let mut id_client_in_bytes = self.id_client.as_bytes().to_vec();
            bytes.push(0x00);
            bytes.push(id_length as u8);
            bytes.append(&mut id_client_in_bytes);
        } else {
            bytes.append(&mut vec![0x00, 0x02, 0x00, 0x00]);
        }
    }

    fn add_password_bytes(&self, flags: &mut u8, bytes: &mut Vec<u8>) {
        if !self.password.is_empty() {
            *flags |= 0b01000000;
            let password_length = self.password.len();
            let mut password_in_bytes = self.password.as_bytes().to_vec();
            bytes.push(0x00);
            bytes.push(password_length as u8);
            bytes.append(&mut password_in_bytes);
        }
    }

    fn add_username_bytes(&self, flags: &mut u8, bytes: &mut Vec<u8>) {
        if !self.user.is_empty() {
            *flags |= 0b10000000;
            let user_length = self.user.len();
            let mut user_in_bytes = self.user.as_bytes().to_vec();
            bytes.push(0x00);
            bytes.push(user_length as u8);
            bytes.append(&mut user_in_bytes);
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
            0x00, 0x00, //Keep Alive
        ];
        self.add_client_id_bytes(&mut bytes);
        self.add_username_bytes(&mut flags, &mut bytes);
        self.add_password_bytes(&mut flags, &mut bytes);
        bytes[8] = flags;
        let length = bytes.len();
        bytes.insert(1, (length - 1) as u8);
        bytes
    }
}