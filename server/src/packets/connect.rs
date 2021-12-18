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
    flags: ConnectFlags,
    payload: ConnectPayload,
    return_code: u8,
}

impl Connect {
    pub fn process_message(bytes: &[u8], packet_manager: &mut PacketManager) -> Result<(), String> {
        if !packet_manager.is_disconnected() {
            Err("Client is already connected".to_string())
        } else {
            let sender_stream = packet_manager.get_sender_stream();
            let sender_user_manager = packet_manager.get_sender_user_manager();

            let connect = Connect::init(bytes, &packet_manager)?;
            packet_manager.set_client_id(connect.get_client_id());
            connect.send_response(sender_stream, sender_user_manager)?;
            packet_manager.connect();
            Ok(())
        }
    }

    fn get_variable_header(
        bytes: &[u8],
        mut return_code: ConnectReturnCode,
    ) -> Result<(&[u8], usize, ConnectReturnCode), String> {
        if bytes.len() == 1 {
            return Err("packet malformed".to_string());
        }

        let bytes_rem_len = &bytes[1..bytes.len()];
        let (readed_index, remaining_length) = save_remaining_length(bytes_rem_len)?;

        if bytes_rem_len.len() != remaining_length + 1 {
            return Err("packet malformed".to_string());
        }

        let init_variable_header = 1 + readed_index;
        let end_variable_header = readed_index + 10;
        let variable_header = &bytes[init_variable_header..end_variable_header + 1];
        check_variable_header_len(variable_header)?;
        return_code = return_code.check_protocol_level(variable_header[6]);

        Ok((variable_header, end_variable_header, return_code))
    }

    fn get_payload(
        bytes: &[u8],
        flags: &ConnectFlags,
        readed_bytes: usize,
        return_code: ConnectReturnCode,
    ) -> Result<(ConnectPayload, ConnectReturnCode), String> {
        let rem_bytes = &bytes[readed_bytes + 1..];
        let payload = ConnectPayload::init(&flags, rem_bytes, return_code)?;
        Ok(payload)
    }

    fn process_keep_alive(
        variable_header: &[u8],
        sender_stream: Sender<StreamType>,
    ) -> Result<(), String> {
        match get_keep_alive(variable_header) {
            Some(some_keep_alive) => handler_keep_alive::init(
                ((some_keep_alive as f64) * 1.5) as u64,
                sender_stream.clone(),
            ),
            None => handler_null_keep_alive::init(sender_stream.clone()),
        }
    }

    fn create_action_with_will_flag(&self, sender_stream: Sender<StreamType>) -> AddUserManager {
        let client_id = self.payload.get_client_id();
        let session_flag = self.flags.get_clean_session_flag();
        let will_topic = self.payload.get_will_topic();
        let will_message = self.payload.get_will_message();
        let will_qos = self.flags.get_will_qos_flag();
        let will_retained_message = self.flags.get_will_retain_flag();
        AddUserManager::init_with_will(
            client_id,
            sender_stream,
            session_flag,
            will_topic,
            will_message,
            Some(will_qos),
            Some(will_retained_message),
        )
    }

    fn create_action_without_will_flag(&self, sender_stream: Sender<StreamType>) -> AddUserManager {
        let client_id = self.payload.get_client_id();
        let session_flag = self.flags.get_clean_session_flag();
        AddUserManager::init_without_will(client_id, sender_stream, session_flag)
    }

    fn create_action_user_manager(&self, sender_stream: Sender<StreamType>) -> UserManagerAction {
        let will_flag = self.flags.get_will_flag();
        let action = if will_flag {
            self.create_action_with_will_flag(sender_stream.clone())
        } else {
            self.create_action_without_will_flag(sender_stream.clone())
        };
        UserManagerAction::AddUserManager(action)
    }

    fn init(bytes: &[u8], packet_manager: &PacketManager) -> Result<Connect, String> {
        let sender_stream = packet_manager.get_sender_stream();
        let sender_user_manager = packet_manager.get_sender_user_manager();
        let return_code = ConnectReturnCode::init();

        println!("0");
        let (variable_header, readed_bytes, return_code) =
            Connect::get_variable_header(bytes, return_code)?;

        println!("1");
        let flags = ConnectFlags::init(&variable_header[7]);
        let (payload, return_code) =
            Connect::get_payload(bytes, &flags, readed_bytes, return_code)?;
        println!("2");
        let connect = Connect {
            flags,
            payload,
            return_code: return_code.apply_validations(),
        };
        println!("3");
        Connect::process_keep_alive(variable_header, sender_stream.clone())?;
        let action = connect.create_action_user_manager(sender_stream.clone());
        if let Err(_err) = sender_user_manager.send(action) {
            Err("Error adding user manager".to_string())
        } else {
            Ok(connect)
        }
    }

    fn send_response(
        &self,
        sender_stream: Sender<StreamType>,
        sender_user_manager: Sender<UserManagerAction>,
    ) -> Result<(), String> {
        let session_present_bit = !(0x01 & self.flags.get_clean_session_flag() as u8);
        let connack_response = [0x20, 0x02, session_present_bit, self.return_code].to_vec();
        if let Err(_msg_error) =
            sender_stream.send((WriteStream, Some(connack_response), None, None))
        {}

        if self.return_code != 0x00 {
            Disconnect::disconnect_user(self.get_client_id(), sender_user_manager, sender_stream);
            Err("".to_string())
        } else {
            Ok(())
        }
    }

    fn get_client_id(&self) -> String {
        self.payload.get_client_id()
    }
}
