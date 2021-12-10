#[cfg(test)]
mod tests {
    use server::flags::connect_flags::ConnectFlags;

    #[test]
    fn should_create_flags_with_username() {
        let flags: u8 = 0x80;
        let connect_flags = ConnectFlags::init(&flags);
        assert_eq!(connect_flags.get_username_flag(), true);
        assert_eq!(connect_flags.get_password_flag(), false);
        assert_eq!(connect_flags.get_will_retain_flag(), false);
        assert_eq!(connect_flags.get_will_flag(), false);
        assert_eq!(connect_flags.get_clean_session_flag(), false);
    }

    #[test]
    fn should_create_flags_with_password() {
        let flags: u8 = 0x40;
        let connect_flags = ConnectFlags::init(&flags);
        assert_eq!(connect_flags.get_username_flag(), false);
        assert_eq!(connect_flags.get_password_flag(), true);
        assert_eq!(connect_flags.get_will_retain_flag(), false);
        assert_eq!(connect_flags.get_will_flag(), false);
        assert_eq!(connect_flags.get_clean_session_flag(), false);
    }

    #[test]
    fn should_create_flags_with_retain() {
        let flags: u8 = 0x20;
        let connect_flags = ConnectFlags::init(&flags);
        assert_eq!(connect_flags.get_username_flag(), false);
        assert_eq!(connect_flags.get_password_flag(), false);
        assert_eq!(connect_flags.get_will_retain_flag(), true);
        assert_eq!(connect_flags.get_will_flag(), false);
        assert_eq!(connect_flags.get_clean_session_flag(), false);
    }

    #[test]
    fn should_create_flags_with_will_flag() {
        let flags: u8 = 0x04;
        let connect_flags = ConnectFlags::init(&flags);
        assert_eq!(connect_flags.get_username_flag(), false);
        assert_eq!(connect_flags.get_password_flag(), false);
        assert_eq!(connect_flags.get_will_retain_flag(), false);
        assert_eq!(connect_flags.get_will_flag(), true);
        assert_eq!(connect_flags.get_clean_session_flag(), false);
    }

    #[test]
    fn should_create_flags_with_clean_session() {
        let flags: u8 = 0x02;
        let connect_flags = ConnectFlags::init(&flags);
        assert_eq!(connect_flags.get_username_flag(), false);
        assert_eq!(connect_flags.get_password_flag(), false);
        assert_eq!(connect_flags.get_will_retain_flag(), false);
        assert_eq!(connect_flags.get_will_flag(), false);
        assert_eq!(connect_flags.get_clean_session_flag(), true);
    }

    #[test]
    fn should_create_flags_with_qos() {
        let flags: u8 = 0x18;
        let connect_flags = ConnectFlags::init(&flags);
        assert_eq!(connect_flags.get_username_flag(), false);
        assert_eq!(connect_flags.get_password_flag(), false);
        assert_eq!(connect_flags.get_will_retain_flag(), false);
        assert_eq!(connect_flags.get_will_flag(), false);
        assert_eq!(connect_flags.get_clean_session_flag(), false);
        assert_eq!(connect_flags.get_will_qos_flag(), 3);
    }
}
