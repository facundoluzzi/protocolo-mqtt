pub struct ConnectFlags {
    username: bool,
    password: bool,
    will_retain: bool,
    will_qos: u8,
    will_flag: bool,
    clean_session: bool,
}

impl ConnectFlags {
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

    pub fn get_username_flag(&self) -> bool {
        self.username
    }

    pub fn get_password_flag(&self) -> bool {
        self.password
    }

    pub fn get_will_retain_flag(&self) -> bool {
        self.will_retain
    }

    pub fn get_will_flag(&self) -> bool {
        self.will_flag
    }

    pub fn get_clean_session_flag(&self) -> bool {
        self.clean_session
    }

    pub fn get_will_qos_flag(&self) -> u8 {
        self.will_qos
    }
}

#[cfg(test)]
mod tests {
    use crate::flags::connect_flags::ConnectFlags;

    #[test]
    fn creacion_correcta_de_flags() {
        let flags: u8 = 0b11000000;
        let connect_flags = ConnectFlags::init(&flags);
        assert_eq!(connect_flags.get_username_flag(), true);
        assert_eq!(connect_flags.get_password_flag(), true);
        assert_eq!(connect_flags.get_will_retain_flag(), false);
        assert_eq!(connect_flags.get_will_flag(), false);
        assert_eq!(connect_flags.get_clean_session_flag(), false);
    }
}
