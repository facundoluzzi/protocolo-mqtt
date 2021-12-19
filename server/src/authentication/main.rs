use std::collections::HashMap;

/// Recibe un nombre de usuario, password y un hash de users ya precargado. Valida que valido.
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
