use crate::flags::flags::Flags;

pub struct ConnectFlags {
    username: bool,
    password: bool,
    will_retain: bool,
    will_qos: u8,
    will_flag: bool,
    clean_session: bool,
}

impl Flags for ConnectFlags {
    fn new(bytes: &u8) -> Box<dyn Flags> {
        Box::new(ConnectFlags {
            will_qos: (0b00011000 & bytes) >> 3,
            username: 0b10000000 & bytes != 0,
            password: 0b01000000 & bytes != 0,
            will_retain: 0b00100000 & bytes != 0,
            will_flag: 0b00000100 & bytes != 0,
            clean_session: 0b00000010 & bytes != 0,
        })
    }

    fn get_username_flag(&self) -> bool {
        self.username
    }

    fn get_password_flag(&self) -> bool {
        self.password
    }

    fn get_will_retain_flag(&self) -> bool {
        self.will_retain
    }

    fn get_will_flag(&self) -> bool {
        self.will_flag
    }

    fn get_clean_session_flag(&self) -> bool {
        self.clean_session
    }

    fn get_will_qos_flag(&self) -> u8 {
        self.will_qos
    }
}
