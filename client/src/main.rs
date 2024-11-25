mod commands;
mod user;

use std::io::{self, Write};
use commands::{is_valid_email_addr, is_valid_password, is_valid_username, Command};
use reqwest::Client;
use user::User;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the real time chat app!");
    println!("Type 'help' to see available commands.");
    // client, input
    let client = Client::new();
    let mut input = String::new();
    let mut user = User::new();

    // get the command
    loop {
        print!("> ");
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;

        match commands::parse_command(&input) {
            Some(Command::Help) => {
                // todo help message
                println!("signup [username] [email] [password]");
                println!("login [username] [password]");
                println!("logout");
                println!("see_active_users");
                println!("create_private_chat []");
                println!("resume_private_chat []");
                println!("create_chat_room []");
                println!("resume_chat_room []");
            }
            Some(Command::Signup { username, email, password }) => {
                // check whether session exists
                if user.session_exists() {
                    println!("You have already logged in!");
                    println!("Please log out first!");
                    continue;
                }

                if !is_valid_username(&username)
                {
                    continue;
                }
                if !is_valid_email_addr(&email)
                {
                    continue;
                }
                if !is_valid_password(&password)
                {
                    continue;
                }

                user.signup(&client, username.to_string(),
                    email.to_string(),
                    password.to_string()).await?;
            }
            Some(Command::Login { username, password }) => {
                // check whether session exists
                if user.session_exists() {
                    println!("You have already logged in!");
                    println!("Please log out first before logging into another account!");
                    continue;
                }

                user.login(&client, &username.to_string(), password.to_string()).await?;
                
                // todo setup websocket
            }
            Some(Command::Logout) => {
                // check whether session exists
                if !user.session_exists() {
                    println!("Please login first!");
                    continue;
                }

                user.logout(&client).await?;
            }
            Some(Command::Quit) => {
                // exit the app
                println!("App is shutting down...");
                println!("Bye!");
                break;
            }
            None => {
                println!("Unknown command. Type 'help' to see available commands.")
            }
        }
    }
    Ok(())
}