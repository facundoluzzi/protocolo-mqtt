use crate::flags::connect_flags::ConnectFlags;
use crate::helper::remaining_length::save_remaining_length;
use crate::helper::status_code::ConnectReturnCode;
use crate::keep_alive::handler_keep_alive;
use crate::keep_alive::handler_null_keep_alive;
use crate::payload::connect_payload::ConnectPayload;
use crate::stream::stream_handler::StreamAction::WriteStream;
use crate::stream::stream_handler::StreamType;
use crate::usermanager::user_manager_action::UserManagerAction::AddUserManager;
use crate::usermanager::user_manager_types::ChannelUserManager;
use crate::variable_header::connect_variable_header::{check_variable_header_len, get_keep_alive};
use std::sync::mpsc::Sender;

pub struct Connect {
    _remaining_length: usize,
    flags: ConnectFlags,
    payload: ConnectPayload,
    status_code: u8,
}

impl Connect {
    pub fn init(
        bytes: &[u8],
        sender_stream: Sender<StreamType>,
        user_manager_sender: Sender<ChannelUserManager>,
    ) -> Result<Connect, String> {
        let mut status_code = ConnectReturnCode::init();

        let bytes_rem_len = &bytes[1..bytes.len()];
        let (readed_index, remaining_length) = save_remaining_length(bytes_rem_len)?;

        let init_variable_header = 1 + readed_index;
        let end_variable_header = readed_index + 10;
        let variable_header = &bytes[init_variable_header..end_variable_header + 1];

        check_variable_header_len(variable_header)?;

        status_code = status_code.check_protocol_level(variable_header[6]);

        let connect_flags = ConnectFlags::init(&variable_header[7]);

        let (payload, mut new_status_code) = ConnectPayload::init(
            &connect_flags,
            &bytes[end_variable_header + 1..init_variable_header + remaining_length],
            status_code,
        )?;

        let username = payload.get_username();
        let password = payload.get_password();
        status_code = new_status_code.check_authentication(username, password);

        let session_flag = connect_flags.get_clean_session_flag();
        let flags = connect_flags;

        let client_id = payload.get_client_id();

        let connect = Connect {
            _remaining_length: remaining_length,
            flags,
            payload,
            status_code: status_code.apply_validations(),
        };

        match get_keep_alive(variable_header) {
            Some(some_keep_alive) => {
                handler_keep_alive::init(some_keep_alive as u64, sender_stream.clone())?
            }
            None => handler_null_keep_alive::init(sender_stream.clone())?,
        }

        if connect.status_code != 0x00 {
            // TODO: Cortar la conexión
            Ok(connect)
        } else {
            match user_manager_sender.send((
                AddUserManager,
                client_id,
                Some(sender_stream),
                Some(session_flag),
                None,
            )) {
                Ok(_) => {}
                Err(err) => {
                    println!("err: {}", err);
                }
            };
            Ok(connect)
        }
    }

    pub fn send_response(
        &self,
        stream: Sender<StreamType>,
        sender_to_disconect: Sender<(String, String)>,
    ) -> Result<(), String> {
        let session_present_bit = !(0x01 & self.flags.get_clean_session_flag() as u8);
        let connack_response = [0x20, 0x02, session_present_bit, self.status_code].to_vec();
        if let Err(_msg_error) = stream.send((WriteStream, Some(connack_response), None, None)) {}

        if self.status_code != 0x00 {
            sender_to_disconect
                .send(("".to_string(), "".to_string()))
                .unwrap();
            Err("".to_string())
        } else {
            Ok(())
        }
    }

    pub fn get_client_id(&self) -> String {
        self.payload.get_client_id()
    }
}
