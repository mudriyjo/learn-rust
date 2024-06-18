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
        is_admin: bool
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
        Some(Commands::List) => {}
        Some(Commands::AddUser { username, password, is_admin }) => {}
        Some(Commands::Delete { username }) => {}
        Some(Commands::UpdatePassword { username, new_password }) => {}
        None => {
            println!("use --help to get option list.")
        }
    }
}
