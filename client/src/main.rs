mod commands;
mod user;

use commands::{is_valid_email_addr, is_valid_password, is_valid_username, Command};
use reqwest::Client;
use user::User;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the real time chat app!");
    println!("Type 'help' to see available commands.");
    // client, input
    let mut rl = rustyline::Editor::<()>::new();
    let client = Client::new();
    let mut user = User::new();

    // get the command
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Err(_) => {
                // logout first
                if user.session_exists() {
                    user.logout(&client).await?;
                }

                // exit the app
                println!("App is shutting down...");
                println!("Bye!");
                break;
            }
            Ok(input) => {
                match commands::parse_command(&input) {
                    Some(Command::Help) => {
                        // todo help message
                        println!("Signup: signup [username] [email] [password]");
                        println!("Login: login [username] [password]");
                        println!("Logout: logout");
                        println!("List all active users: list-all");
                        println!("Check the status based on username: check [username]");
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

                        if !is_valid_username(&username) || !is_valid_email_addr(&email) || !is_valid_password(&password)
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
                    Some(Command::ListActiveUsers) => {
                        // check whether session exists
                        if !user.session_exists() {
                            println!("Please login first!");
                            continue;
                        }

                        user.list_active_users(&client).await?;
                    }
                    Some(Command::CheckUserStatus { username }) => {
                        // check whether session exists
                        if !user.session_exists() {
                            println!("Please login first!");
                            continue;
                        }
                    }
                    Some(Command::Quit) => {
                        // logout first
                        if user.session_exists() {
                            user.logout(&client).await?;
                        }

                        // exit the app
                        println!("App is shutting down...");
                        println!("Bye!");
                        break;
                    }
                    None => {
                        println!("Unknown command. Type 'help' to see available commands.")
                    }
                }
                rl.add_history_entry(input.clone());
            }
        }
    }
    
    Ok(())
}