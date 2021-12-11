pub struct ConnackFlags {
    _session_present_flag: bool,
}

impl ConnackFlags {
    pub fn init(bytes: &u8) -> ConnackFlags {
        ConnackFlags {
            _session_present_flag: 0x01 & bytes != 0,
        }
    }
    pub fn _get_session_present_flag(&self) -> bool {
        self._session_present_flag
    }
}

#[cfg(test)]
mod tests {
    use crate::helper::flags::connack_flags::ConnackFlags;

    #[test]
    fn create_connack_flags_with_session_present() {
        let flags: u8 = 0x01;
        let connect_flags = ConnackFlags::init(&flags);
        assert_eq!(connect_flags._get_session_present_flag(), true);
    }
    #[test]
    fn create_connack_flags_without_session_present() {
        let flags: u8 = 0x00;
        let connect_flags = ConnackFlags::init(&flags);
        assert_eq!(connect_flags._get_session_present_flag(), false);
    }
}
