#[cfg(test)]
mod tests {
    use server::helper::status_code::ConnectReturnCode;

    #[test]
    fn return_code_is_success() {
        let connect_return_code = ConnectReturnCode::init();
        assert_eq!(connect_return_code.apply_validations(), 0x00);
    }

    #[test]
    fn return_code_is_success_complete_validation() {
        let connect_return_code = ConnectReturnCode::init()
            .check_protocol_level(4)
            .check_client_identifier(1)
            .check_server_unavailable()
            .check_malformed_username("user1".to_string())
            .check_malformed_password("pass1".to_string())
            .check_authentication(Some(&"user1".to_string()), Some(&"pass1".to_string()));
        assert_eq!(connect_return_code.apply_validations(), 0x00);
    }

    #[test]
    fn return_code_is_unacceptable_protocol() {
        let connect_return_code = ConnectReturnCode::init()
            .check_protocol_level(2)
            .check_client_identifier(1)
            .check_server_unavailable()
            .check_malformed_username("user1".to_string())
            .check_malformed_password("pass1".to_string())
            .check_authentication(Some(&"user1".to_string()), Some(&"pass1".to_string()));
        assert_eq!(connect_return_code.apply_validations(), 0x01);
    }

    #[test]
    fn return_code_is_malformed_username() {
        let connect_return_code = ConnectReturnCode::init()
            .check_protocol_level(4)
            .check_client_identifier(1)
            .check_server_unavailable()
            .check_malformed_username("".to_string())
            .check_malformed_password("pass1".to_string())
            .check_authentication(Some(&"user1".to_string()), Some(&"pass1".to_string()));
        assert_eq!(connect_return_code.apply_validations(), 0x04);
    }

    #[test]
    fn return_code_is_malformed_password() {
        let connect_return_code = ConnectReturnCode::init()
            .check_protocol_level(4)
            .check_client_identifier(1)
            .check_server_unavailable()
            .check_malformed_username("213".to_string())
            .check_malformed_password("".to_string())
            .check_authentication(Some(&"user1".to_string()), Some(&"pass1".to_string()));
        assert_eq!(connect_return_code.apply_validations(), 0x04);
    }

    #[test]
    fn return_code_is_not_authorized_wrong_username() {
        let connect_return_code = ConnectReturnCode::init()
            .check_protocol_level(4)
            .check_client_identifier(1)
            .check_server_unavailable()
            .check_malformed_username("user1".to_string())
            .check_malformed_password("pass1".to_string())
            .check_authentication(Some(&"user".to_string()), Some(&"pass1".to_string()));
        assert_eq!(connect_return_code.apply_validations(), 0x05);
    }

    #[test]
    fn return_code_is_not_authorized_wrong_password() {
        let connect_return_code = ConnectReturnCode::init()
            .check_protocol_level(4)
            .check_client_identifier(1)
            .check_server_unavailable()
            .check_malformed_username("user1".to_string())
            .check_malformed_password("pass1".to_string())
            .check_authentication(Some(&"user".to_string()), Some(&"pass1".to_string()));
        assert_eq!(connect_return_code.apply_validations(), 0x05);
    }

    #[test]
    fn return_code_is_success_password_empty() {
        let connect_return_code = ConnectReturnCode::init()
            .check_protocol_level(4)
            .check_client_identifier(1)
            .check_server_unavailable()
            .check_malformed_username("user1".to_string())
            .check_malformed_password("pass1".to_string())
            .check_authentication(None, None);
        assert_eq!(connect_return_code.apply_validations(), 0x00);
    }

    #[test]
    fn return_code_is_not_authorized_empty_password() {
        let connect_return_code = ConnectReturnCode::init()
            .check_protocol_level(4)
            .check_client_identifier(1)
            .check_server_unavailable()
            .check_malformed_username("user1".to_string())
            .check_malformed_password("pass1".to_string())
            .check_authentication(Some(&"user1".to_string()), None);
        assert_eq!(connect_return_code.apply_validations(), 0x05);
    }

    #[test]
    fn return_code_is_not_authorized_empty_username() {
        let connect_return_code = ConnectReturnCode::init()
            .check_protocol_level(4)
            .check_client_identifier(1)
            .check_server_unavailable()
            .check_malformed_username("user1".to_string())
            .check_malformed_password("pass1".to_string())
            .check_authentication(None, Some(&"pass1".to_string()));
        assert_eq!(connect_return_code.apply_validations(), 0x05);
    }
}
