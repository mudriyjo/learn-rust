use once_cell::sync::Lazy;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

static USERS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(get_users()));

fn get_users() -> Vec<String> {
    vec!["Alice".to_string(), "Bob".to_string()]
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.trim().to_string()
}

fn main() {
    thread::spawn(|| loop {
        println!("List of all users, if you need exit press q...");
        let users = USERS.read().unwrap();
        println!("{users:?}");
        std::thread::sleep(Duration::from_secs(3));
    });

    loop {
        let name = read_line();
        if name == "q" {
            break;
        } else {
            let mut lock = USERS.write().unwrap();
            lock.push(name);
        }
    }
}
