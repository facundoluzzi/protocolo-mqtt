use crate::flags::trait_flags::Flags;

pub struct ConnackFlags {
    username: bool,
    password: bool,
    will_retain: bool,
    will_qos: u8,
    will_flag: bool,
    clean_session: bool,
}

impl Flags for ConnackFlags {
    fn init(bytes: &u8) -> Box<dyn Flags> {
        Box::new(ConnectFlags {
            will_qos: (0b00011000 & bytes) >> 3,
        })
    }

    fn get_username_flag(&self) -> bool {
        self.username
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flags::connack_flags::ConnackFlags;

    #[test]
    fn creacion_correcta_de_flags() {
        let flags: u8 = 0b11000000;
        let connect_flags = ConnackFlags::init(&flags);
        assert_eq!(connect_flags.get_username_flag(), true);
        assert_eq!(connect_flags.get_password_flag(), true);
        assert_eq!(connect_flags.get_will_retain_flag(), false);
        assert_eq!(connect_flags.get_will_flag(), false);
        assert_eq!(connect_flags.get_clean_session_flag(), false);
    }
}
