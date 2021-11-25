use crate::authentication::main::is_authenticated;
use crate::helper::file_handler::get_lines_as_key_values;

#[derive(Clone, Copy)]
pub enum ReturnCode {
    Success,
    UnacceptableProtocol,
    MalformedData,
    NotAuthorized,
}

pub struct ConnectReturnCode {
    status_code: ReturnCode,
}

impl ConnectReturnCode {
    pub fn init() -> Self {
        ConnectReturnCode {
            status_code: ReturnCode::Success,
        }
    }

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

    pub fn check_client_identifier(&self, _client_id: u8) -> ConnectReturnCode {
        // TODO: implementar esto
        ConnectReturnCode {
            status_code: self.status_code,
        }
    }

    pub fn check_server_unavailable(&self) -> ConnectReturnCode {
        // TODO: implementar esto
        ConnectReturnCode {
            status_code: self.status_code,
        }
    }

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

    pub fn check_authentication(
        &mut self,
        username: Option<&String>,
        password: Option<&String>,
    ) -> ConnectReturnCode {
        self.status_code = match self.status_code {
            ReturnCode::Success => match (username.as_ref(), password.as_ref()) {
                (Some(uname), Some(pass)) => {
                    let credentials = get_lines_as_key_values("./credenciales.txt".to_string());
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

    pub fn apply_validations(self) -> u8 {
        match self.status_code {
            ReturnCode::Success => 0x00,
            ReturnCode::UnacceptableProtocol => 0x01,
            ReturnCode::MalformedData => 0x04,
            ReturnCode::NotAuthorized => 0x05,
        }
    }
}
