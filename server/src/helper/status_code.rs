use crate::authentication::main::is_authenticated;
use crate::enums::user_manager::user_manager_action::UserManagerAction;
use crate::enums::user_manager::valid_client_id_user_manager::ValidClientIdUserManager;
use crate::helper::file_handler::get_lines_as_key_values;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

/// contiene los diferentes codigos de respuesta. Validos y no validos.
#[derive(Clone, Copy)]
pub enum ReturnCode {
    Success,
    UnacceptableProtocol,
    MalformedData,
    NotAuthorized,
    InvalidClientId,
}

/// contiene el enum ReturnCode
pub struct ConnectReturnCode {
    status_code: ReturnCode,
}

impl ConnectReturnCode {
    /// Constructor del struct
    pub fn init() -> Self {
        ConnectReturnCode {
            status_code: ReturnCode::Success,
        }
    }

    /// Valida la version del protocolo si no hay un error previo
    pub fn check_protocol_level(&mut self, protocol_level: u8) -> ConnectReturnCode {
        self.status_code = match self.status_code {
            ReturnCode::Success => {
                if protocol_level != 4 {
                    ReturnCode::UnacceptableProtocol
                } else {
                    self.status_code
                }
            }
            _ => self.status_code,
        };
        ConnectReturnCode {
            status_code: self.status_code,
        }
    }

    fn check_client_id_value(&self, receiver: Receiver<bool>) -> ReturnCode {
        if let Ok(client_id_is_valid) = receiver.recv() {
            if !client_id_is_valid {
                ReturnCode::InvalidClientId
            } else {
                self.status_code
            }
        } else {
            ReturnCode::InvalidClientId
        }
    }

    /// Valida el client id si no hay un error previo
    pub fn check_client_id(
        &self,
        client_id: Option<String>,
        sender_user_manager: Sender<UserManagerAction>,
    ) -> ConnectReturnCode {
        if client_id.is_none() {
            return ConnectReturnCode {
                status_code: ReturnCode::InvalidClientId,
            };
        }
        let status = match client_id {
            Some(client_id_x) => {
                let (sender, receiver): (Sender<bool>, Receiver<bool>) = channel();
                let valid_client_id = ValidClientIdUserManager::init(client_id_x, sender);
                if let Err(_err) =
                    sender_user_manager.send(UserManagerAction::ValidClientId(valid_client_id))
                {
                    ReturnCode::InvalidClientId
                } else {
                    self.check_client_id_value(receiver)
                }
            }
            None => ReturnCode::InvalidClientId,
        };

        ConnectReturnCode {
            status_code: status,
        }
    }

    /// Valida que el usuario no sea vacío, si no hay un error previo
    pub fn check_malformed_username(&mut self, username: String) -> ConnectReturnCode {
        self.status_code = match self.status_code {
            ReturnCode::Success => {
                if username.is_empty() {
                    ReturnCode::MalformedData
                } else {
                    self.status_code
                }
            }
            _ => self.status_code,
        };
        ConnectReturnCode {
            status_code: self.status_code,
        }
    }

    /// Valida el formato de la password, si no hay un error previo
    pub fn check_malformed_password(&mut self, password: String) -> ConnectReturnCode {
        self.status_code = match self.status_code {
            ReturnCode::Success => {
                if password.is_empty() || password.len() >= 65535 {
                    ReturnCode::MalformedData
                } else {
                    self.status_code
                }
            }
            _ => self.status_code,
        };
        ConnectReturnCode {
            status_code: self.status_code,
        }
    }

    /// Valida la autenticación, si no hay un error previo
    pub fn check_authentication(
        &mut self,
        username: Option<String>,
        password: Option<String>,
    ) -> ConnectReturnCode {
        self.status_code = match self.status_code {
            ReturnCode::Success => match (username.as_ref(), password.as_ref()) {
                (Some(uname), Some(pass)) => {
                    let credentials =
                        get_lines_as_key_values("../server/credenciales.txt".to_string());
                    if is_authenticated(uname.to_string(), pass.to_string(), credentials) {
                        ReturnCode::Success
                    } else {
                        ReturnCode::NotAuthorized
                    }
                }
                (None, None) => ReturnCode::Success,
                _ => ReturnCode::NotAuthorized,
            },
            _ => self.status_code,
        };
        ConnectReturnCode {
            status_code: self.status_code,
        }
    }

    /// Devuelve el primer error capturado o 0x00 si no hubo errores
    pub fn apply_validations(self) -> u8 {
        match self.status_code {
            ReturnCode::Success => 0x00,
            ReturnCode::UnacceptableProtocol => 0x01,
            ReturnCode::InvalidClientId => 0x02,
            ReturnCode::MalformedData => 0x04,
            ReturnCode::NotAuthorized => 0x05,
        }
    }
}
