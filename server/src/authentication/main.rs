use crate::helper::file_handler::get_lines_as_key_values;

pub fn is_authenticated(username: String, password: String) -> bool {
    let path = "./credenciales.txt";
    let users = get_lines_as_key_values(path.to_owned());
    match users.get(&username) {
        Some(user_password) => *user_password.to_string() == *password,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_authenticated_exitosa() {
        assert_eq!(
            is_authenticated("user1".to_string(), "pass1".to_string()),
            true
        );
    }

    #[test]
    fn is_authenticated_fallida() {
        assert_eq!(
            is_authenticated("user1".to_string(), "pass2".to_string()),
            false
        );
    }

    #[test]
    fn is_authenticated_fallida_usuario_no_existe() {
        assert_eq!(
            is_authenticated("usern".to_string(), "pass1".to_string()),
            false
        );
    }
}
