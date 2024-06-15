use authentication::{read_console, login};
fn main() {
    let mut attempt = 3;
    loop {
        println!("Write your name:");
        let name = read_console();
        println!("Write your password:");
        let password = read_console();
        if login(&name, &password) {
            println!("Welcome!");
            break;
        } else if attempt > 3 {
            println!("Too many attempts...");
            break;
        } else {
            println!("Wrong name or password....");
            attempt += 1;
        }
    }
}
