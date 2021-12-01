#[cfg(test)]
mod tests {
    use std::{collections::HashMap, iter::FromIterator};

    use server::authentication::main::is_authenticated;

    fn make_credentials() -> HashMap<String, String> {
        let a: Vec<(String, String)> = vec![
            ("user1".to_string(), "pass1".to_string()),
            ("user2".to_string(), "pass2".to_string()),
            ("user3".to_string(), "pass3".to_string()),
            ("ALTEGO".to_string(), "ALT".to_string()),
        ];
        let users: HashMap<String, String> = HashMap::from_iter(a);
        users
    }

    #[test]
    fn should_authenticate_successfully() {
        let users = make_credentials();
        assert_eq!(
            is_authenticated("user1".to_string(), "pass1".to_string(), users),
            true
        );
    }

    #[test]
    fn should_fail_the_authentication() {
        let users = make_credentials();
        assert_eq!(
            is_authenticated("user1".to_string(), "pass2".to_string(), users),
            false
        );
    }

    #[test]
    fn should_fail_the_authentication_when_user_dont_exist() {
        let users = make_credentials();
        assert_eq!(
            is_authenticated("usern".to_string(), "pass1".to_string(), users),
            false
        );
    }
}
