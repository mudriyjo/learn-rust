use authentication::{login, read_console, LoginAction, LoginRole};
fn main() {
    let mut attempt = 3;
    loop {
        println!("Write your name:");
        let name = read_console();
        println!("Write your password:");
        let password = read_console();
        match login(&name, &password) {
            Some(LoginAction::Granted(role)) => {
                match role {
                    LoginRole::Admin => println!("Welcome Admin!"),
                    LoginRole::User => println!("Welcome User!")
                }
                break;
            }
            Some(LoginAction::Denied) => {
                println!("Wrong name or password....");
                attempt += 1;
                if attempt > 3 {
                    break;
                }
            }
            None => {
                println!("New user tried login!");
                break;
            }
        }
        // if login(&name, &password) {
        //     println!("Welcome!");
        //     break;
        // } else if attempt > 3 {
        //     println!("Too many attempts...");
        //     break;
        // } else {
        //     println!("Wrong name or password....");
        //     attempt += 1;
        // }
    }
}
