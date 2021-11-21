use crate::flags::connect_flags::ConnectFlags;
use crate::helper::remaining_length::save_remaining_length;
use crate::helper::status_code::ConnectReturnCode;
use crate::helper::user_manager::UserManager;
use crate::payload::connect_payload::ConnectPayload;

use std::io::Write;
use std::net::TcpStream;

pub struct Connect {
    _remaining_length: usize,
    flags: ConnectFlags,
    payload: ConnectPayload,
    status_code: u8,
}

impl Connect {
    pub fn init(bytes: &[u8], stream: &TcpStream, mut user_manager: UserManager) -> Connect {
        let mut status_code = ConnectReturnCode::init();
        let bytes_rem_len = &bytes[1..bytes.len()];
        let (readed_index, remaining_length) = save_remaining_length(bytes_rem_len).unwrap();

        let init_variable_header = 1 + readed_index;
        let end_variable_header = readed_index + 10;
        let variable_header = &bytes[init_variable_header..end_variable_header + 1];

        status_code = status_code.check_protocol_level(variable_header[6]);

        let connect_flags = ConnectFlags::init(&variable_header[7]);

        let (payload, new_status_code) = ConnectPayload::init(
            &connect_flags,
            &bytes[end_variable_header + 1..init_variable_header + remaining_length],
            status_code,
        );

        status_code = new_status_code;
        let username = payload.get_username();
        let password = payload.get_password();
        status_code = status_code.check_authentication(username, password);

        let flags = connect_flags;

        let client_id = payload.get_client_id();

        let connect = Connect {
            _remaining_length: remaining_length,
            flags,
            payload: payload,
            status_code: status_code.apply_validations(),
        };

        if connect.status_code != 0x00 {
            connect
        } else {
            if let Some(mut usuario) = user_manager.find_user(connect.get_client_id()) {
                usuario.reconnect(stream.try_clone().unwrap());
            } else {
                user_manager.add(client_id, stream.try_clone().unwrap());
            };
            connect
        }
    }

    pub fn get_type(&self) -> String {
        "connect".to_owned()
    }

    pub fn send_response(&self, mut stream: &TcpStream) {
        let session_present_bit = !(0x01 & self.flags.get_clean_session_flag() as u8);
        let connack_response = [0x20, 0x02, session_present_bit, self.status_code];
        if let Err(msg_error) = stream.write(&connack_response) {
            println!("Error in sending response: {}", msg_error);
        }
    }

    pub fn get_client_id(&self) -> String {
        self.payload.get_client_id()
    }
}


