use std::{collections::HashMap, fs, path::{Path, PathBuf}};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

pub fn greet_user(name: &str) -> String {
    format!("Hello {name}!")
}

pub fn read_console() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Stdin not working!");
    input.trim().to_string()
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        Self {
            username: username.to_string(),
            password: hash_password(password),
            role: role
        }
    }

    pub fn new_is_admin(username: &str, password: &str, isAdmin: bool) -> User {
        Self {
            username: username.to_string(),
            password: hash_password(password),
            role: if isAdmin {LoginRole::Admin} else {LoginRole::User}
        }
    }
}

fn get_admins() -> HashMap<String, User> {
    get_user()
    .into_iter()
    .filter(|user| user.1.role == LoginRole::Admin)
    .collect()
}

fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

fn get_default_user() -> HashMap<String, User> {
    let mut users: HashMap<String, User> = HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));
    users.insert("bob".to_string(), User::new("bob", "password", LoginRole::User));
    users
}

fn save_users(users: &HashMap<String, User>) {
    let json_users = serde_json::to_string(&users).unwrap();
    fs::write(get_path().as_path(), json_users).unwrap();
}

fn get_path() -> PathBuf {
    let mut path = PathBuf::new();
    path.push("users.json");
    path
}

pub fn get_user() -> HashMap<String, User> {
    let users_path_buf = get_path();
    let users_path = users_path_buf.as_path();
    if Path::exists(users_path) {
        let json_users = fs::read_to_string(users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&json_users).unwrap();
        users
    } else {
        let default_users = get_default_user();
        save_users(&default_users);
        default_users
    }
}

pub fn delete_user(username: &str) {
    let mut users = get_user();
    if users.contains_key(username) {
        users.remove(username);
        save_users(&users);
    } else {
        println!("user doesn't exist!");
    }
}

pub fn update_user(username: &str, f: &impl Fn(&mut User)) {
    let mut users = get_user();
    if let Some(user) = users.get_mut(username) {
        f(user);
        save_users(&users)
    } else {
        println!("user doesn't exist!")
    }
}

pub fn add_user(username: &str, password: &str, isAdmin: bool) {
    let mut users = get_user();
    if let Some(_) = users.get(username) {
        println!("user with this username already exist!")
    } else {
        let new_user = User::new_is_admin(username, password, isAdmin);
        users.insert(username.to_string(), new_user);
        save_users(&users)
    }
}

pub fn login(name: &str, password: &str) -> Option<LoginAction> {
    let username = name.to_lowercase();
    if let Some(user) = get_user().get(&username) {
        if user.password == hash_password(password) {
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
