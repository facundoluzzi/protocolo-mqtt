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