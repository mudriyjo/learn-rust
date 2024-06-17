use std::{collections::HashMap};

pub fn greet_user(name: &str) -> String {
    format!("Hello {name}!")
}

pub fn read_console() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Stdin not working!");
    input.trim().to_string()
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginRole {
    Admin,
    User
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied
}

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            role: role
        }
    }
}

fn get_admins() -> HashMap<String, User> {
    get_user()
    .into_iter()
    .filter(|user| user.1.role == LoginRole::Admin)
    .collect()
}

fn get_user() -> HashMap<String, User> {
    let mut users: HashMap<String, User> = HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));
    users.insert("bob".to_string(), User::new("bob", "password", LoginRole::User));
    users
}

pub fn login(name: &str, password: &str) -> Option<LoginAction> {
    let username = name.to_lowercase();
    if let Some(user) = get_user().get(&username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!("Hello John!", greet_user("John"));
    }

    #[test]
    fn test_login() {
        assert_eq!(login("admin", "password"), Some(LoginAction::Granted(LoginRole::Admin)));
        assert_eq!(login("AdMiN", "password"), Some(LoginAction::Granted(LoginRole::Admin)));
        assert_eq!(login("not-admin", "password"), None);
        assert_eq!(login("admin", "not-password"), Some(LoginAction::Denied));
        assert_eq!(login("bob", "not-password"), Some(LoginAction::Denied));
        assert_eq!(login("bob", "password"), Some(LoginAction::Granted(LoginRole::User)));
    }
}
