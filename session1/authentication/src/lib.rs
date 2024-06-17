pub fn greet_user(name: &str) -> String {
    format!("Hello {name}!")
}

pub fn read_console() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Stdin not working!");
    input.trim().to_string()
}
pub enum LoginRole {
    Admin,
    User
}

pub enum LoginAction {
    Granted(LoginRole),
    Denied
}

pub fn login(name: &str, password: &str) -> Option<LoginAction> {
    let name = name.to_lowercase();
    if name != "admin" && name != "bob" {
        return None;
    }

    if name == "admin" && password == "password" {
        Some(LoginAction::Granted(LoginRole::Admin))
    } else if name == "bob" && password == "password" {
        Some(LoginAction::Granted(LoginRole::User))
    } else {
        Some(LoginAction::Denied)
    }
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
        // assert!(login("admin", "password"));
        // assert!(login("AdMiN", "password"));
        // assert!(!login("not-admin", "password"));
        // assert!(!login("admin", "not-password"));
    }
}
