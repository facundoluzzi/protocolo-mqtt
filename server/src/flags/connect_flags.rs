pub struct ConnectFlags {
    username: bool,
    password: bool,
    will_retain: bool,
    will_qos: u8,
    will_flag: bool,
    clean_session: bool,
}

impl ConnectFlags {

    /// Recibe un byte y crea el paquete de connect flags
    pub fn init(bytes: &u8) -> ConnectFlags {
        ConnectFlags {
            will_qos: (0x18 & bytes) >> 3,
            username: 0x80 & bytes != 0,
            password: 0x40 & bytes != 0,
            will_retain: 0x20 & bytes != 0,
            will_flag: 0x04 & bytes != 0,
            clean_session: 0x02 & bytes != 0,
        }
    }

    /// Obtiene el flag de username
    pub fn get_username_flag(&self) -> bool {
        self.username
    }

    /// Obtiene el flag de password
    pub fn get_password_flag(&self) -> bool {
        self.password
    }

    /// Obtiene el flag de will retain
    pub fn get_will_retain_flag(&self) -> bool {
        self.will_retain
    }

    /// Obtiene el flag de will message
    pub fn get_will_flag(&self) -> bool {
        self.will_flag
    }

    /// Obtiene el flag de clean session
    pub fn get_clean_session_flag(&self) -> bool {
        self.clean_session
    }

    /// obtiene el flag de qos
    pub fn get_will_qos_flag(&self) -> u8 {
        self.will_qos
    }
}
