use authentication::{get_user, hash_password, User};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct App {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Show all user in login manager
    List,
    /// Add new user to login manager
    AddUser {
        /// User's username, should be uniq
        username: String,
        /// User's password (plaintext)
        password: String,
        /// Is user Admin or regular User
        #[arg(long)]
        is_admin: Option<bool>
    },
    /// Delete user from login manager
    Delete {
        /// User's username
        username: String
    },
    /// Update user'spassword in login manager
    UpdatePassword {
        /// User's username
        username: String,
        /// User's new password
        new_password: String
    }

}

fn main() {
    let parser = App::parse();
    match parser.command {
        Some(Commands::List) => {
            println!("{:=<40}", "=");
            println!("{:<20}{:<20}", "Username", "Role");
            println!("{:=<40}", "=");
            get_user()
            .iter()
            .for_each(|user| println!("{:<20}{:<20?}", user.1.username, user.1.role));
            println!("{:=<40}", "=");
        }
        Some(Commands::AddUser { username, password, is_admin }) => {
            authentication::add_user(&username, &password, is_admin.unwrap_or(false))
        }
        Some(Commands::Delete { username }) => {
            authentication::delete_user(&username)
        }
        Some(Commands::UpdatePassword { username, new_password }) => {
            authentication::update_user(&username, &|user: &mut User| {
                user.password.clone_from(&hash_password(&new_password));
            })
        }
        None => {
            println!("use --help to get option list.")
        }
    }
}
