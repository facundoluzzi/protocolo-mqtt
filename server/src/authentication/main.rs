use std::collections::HashMap;

pub fn is_authenticated(
    username: String,
    password: String,
    users: HashMap<String, String>,
) -> bool {
    match users.get(&username) {
        Some(user_password) => *user_password.to_string() == *password,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_credentials() -> HashMap<String, String> {
        let users = HashMap::from([
            ("user1".to_string(), "pass1".to_string()),
            ("user2".to_string(), "pass2".to_string()),
            ("user3".to_string(), "pass3".to_string()),
            ("ALTEGO".to_string(), "ALT".to_string()),
        ]);
        users
    }

    #[test]
    fn is_authenticated_exitosa() {
        let users = make_credentials();
        assert_eq!(
            is_authenticated("user1".to_string(), "pass1".to_string(), users),
            true
        );
    }

    #[test]
    fn is_authenticated_fallida() {
        let users = make_credentials();
        assert_eq!(
            is_authenticated("user1".to_string(), "pass2".to_string(), users),
            false
        );
    }

    #[test]
    fn is_authenticated_fallida_usuario_no_existe() {
        let users = make_credentials();
        assert_eq!(
            is_authenticated("usern".to_string(), "pass1".to_string(), users),
            false
        );
    }
}
