use crate::enums::user_manager::add_user_manager::AddUserManager;
use crate::enums::user_manager::user_manager_action::UserManagerAction;
use crate::flags::connect_flags::ConnectFlags;
use crate::helper::remaining_length::save_remaining_length;
use crate::helper::status_code::ConnectReturnCode;
use crate::keep_alive::handler_keep_alive;
use crate::keep_alive::handler_null_keep_alive;
use crate::packets::packet_manager::PacketManager;
use crate::payload::connect_payload::ConnectPayload;
use crate::stream::stream_handler::StreamAction::WriteStream;
use crate::stream::stream_handler::StreamType;
use crate::variable_header::connect_variable_header::{check_variable_header_len, get_keep_alive};
use std::sync::mpsc::Sender;

use super::disconnect::Disconnect;

pub struct Connect {
    _remaining_length: usize,
    flags: ConnectFlags,
    payload: ConnectPayload,
    status_code: u8,
}

impl Connect {
    pub fn process_message(bytes: &[u8], packet_manager: &mut PacketManager) -> Result<(), String> {
        if !packet_manager.is_disconnected() {
            Err("Client is already connected".to_string())
        } else {
            let sender_stream = packet_manager.get_sender_stream();
            let sender_user_manager = packet_manager.get_sender_user_manager();

            let connect = Connect::init(bytes, sender_stream.clone(), sender_user_manager.clone())?;
            packet_manager.set_client_id(connect.get_client_id());
            connect.send_response(sender_stream.clone(), sender_user_manager.clone())?;
            Ok(())
        }
    }

    pub fn init(
        bytes: &[u8],
        sender_stream: Sender<StreamType>,
        user_manager_sender: Sender<UserManagerAction>,
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
        let will_flag = connect_flags.get_will_flag();
        let client_id = payload.get_client_id();
        let will_topic = payload.get_will_topic();
        let will_message = payload.get_will_message();
        let will_qos = connect_flags.get_will_qos_flag();
        let will_retained_message = connect_flags.get_will_retain_flag();
        let flags = connect_flags;

        let connect = Connect {
            _remaining_length: remaining_length,
            flags,
            payload,
            status_code: status_code.apply_validations(),
        };

        match get_keep_alive(variable_header) {
            Some(some_keep_alive) => handler_keep_alive::init(
                ((some_keep_alive as f64) * 1.5) as u64,
                sender_stream.clone(),
            )?,
            None => handler_null_keep_alive::init(sender_stream.clone())?,
        }

        if connect.status_code != 0x00 {
            // TODO: Cortar la conexión
            Ok(connect)
        } else {
            let action: UserManagerAction;
            if will_flag {
                action = UserManagerAction::AddUserManager(AddUserManager::init(
                    client_id,
                    sender_stream,
                    session_flag,
                    will_topic,
                    will_message,
                    Some(will_qos),
                    Some(will_retained_message),
                ));
            } else {
                action = UserManagerAction::AddUserManager(AddUserManager::init(
                    client_id,
                    sender_stream,
                    session_flag,
                    None,
                    None,
                    None,
                    None,
                ));
            }
            match user_manager_sender.send(action) {
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
        sender_stream: Sender<StreamType>,
        sender_user_manager: Sender<UserManagerAction>,
    ) -> Result<(), String> {
        let session_present_bit = !(0x01 & self.flags.get_clean_session_flag() as u8);
        let connack_response = [0x20, 0x02, session_present_bit, self.status_code].to_vec();
        if let Err(_msg_error) =
            sender_stream.send((WriteStream, Some(connack_response), None, None))
        {}

        if self.status_code != 0x00 {
            Disconnect::disconnect_user(self.get_client_id(), sender_user_manager, sender_stream);
            Err("".to_string())
        } else {
            Ok(())
        }
    }

    pub fn get_client_id(&self) -> String {
        self.payload.get_client_id()
    }
}
