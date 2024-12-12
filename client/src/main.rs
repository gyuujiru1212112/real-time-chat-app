mod commands;
mod user;
mod common;

use commands::{is_valid_email_addr, is_valid_password, is_valid_username, Command};
use common::{print_help_msg_after_login, print_help_msg_by_default, print_session_exists_error_msg, print_session_not_exist_error_msg, print_msg, print_warning_error_msg};
use reqwest::Client;
use user::User;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_msg("Welcome to the real time chat app!");
    print_msg("Type 'help' to see available commands.");
    // client, input
    let history_file = "history.txt";
    let mut rl = rustyline::Editor::<()>::new();
    let _ = rl.load_history(history_file);

    let mut current_mode = "main";
    let client = Client::new();
    let mut user = User::new();
    let mut prompt = String::from(">> ");

    // get the command
    loop {
        let readline = rl.readline(&prompt);
        match readline {
            Err(_) => {
                // logout first
                if user.session_exists() {
                    user.logout(&client).await?;
                }

                // exit the app
                print_msg("App is shutting down...");
                print_msg("Bye!");
                break;
            }
            Ok(input) => {
                rl.add_history_entry(input.clone());

                match current_mode {
                    "main" => {
                        match commands::parse_command(&input) {
                            Some(Command::Help) => {
                                if user.session_exists() {
                                    print_help_msg_after_login();
                                } else {
                                    print_help_msg_by_default();
                                }
                            }
                            Some(Command::Signup { username, email, password }) => {
                                // check whether session exists
                                if user.session_exists() {
                                    print_warning_error_msg("You have already logged in!");
                                    print_warning_error_msg("Please log out first!");
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
                                    print_session_exists_error_msg();
                                    continue;
                                }
        
                                let res = user.login(&client, &username.to_string(), password.to_string()).await?;
                                if res {
                                    prompt = format!("{} >> ", user.get_user_name());
                                }
                            }
                            Some(Command::Logout) => {
                                // check whether session exists
                                if !user.session_exists() {
                                    print_session_not_exist_error_msg();
                                    continue;
                                }
        
                                let res = user.logout(&client).await?;
                                if res {
                                    prompt = String::from(">> ");
                                }
                            }
                            Some(Command::ListUsers) => {
                                // check whether session exists
                                if !user.session_exists() {
                                    print_session_not_exist_error_msg();
                                    continue;
                                }
        
                                user.list_users(&client).await?;
                            }
                            Some(Command::CheckUserStatus { username }) => {
                                // check whether session exists
                                if !user.session_exists() {
                                    print_session_not_exist_error_msg();
                                    continue;
                                }
        
                                user.check_user_status(&client, username).await?;
                            }
                            Some(Command::CreatePrivateChat { with_user }) => {
                                // check whether session exists
                                if !user.session_exists() {
                                    print_session_not_exist_error_msg();
                                    continue;
                                }
                                let res = user.create_private_chat(&client, with_user.clone()).await?;
                                if res {
                                    current_mode = "child";
                                    let enter_msg = format!("Entering private chat with {}...", with_user);
                                    print_msg(&enter_msg);
                                    prompt = format!("Me ({}): ", user.get_user_name());
                                }
                            }
                            Some(Command::ListAllRecipients) => {
                                // check whether session exists
                                if !user.session_exists() {
                                    print_session_not_exist_error_msg();
                                    continue;
                                }
                                user.list_all_recipients(&client).await?;
                            }
                            Some(Command::CreateChatRoom { name, users }) => {
                                if !user.session_exists() {
                                    print_session_not_exist_error_msg();
                                    continue;
                                }
                                let res = user.create_chat_room(&client, name.clone(), &users).await?;
                                if res {
                                    current_mode = "child";
                                    let enter_msg = format!("Entering chat room {}...", name);
                                    print_msg(&enter_msg);
                                    prompt = format!("Me ({}): ", user.get_user_name());
                                }
                            }
                            Some(Command::ListAllChatRooms) => {
                                if !user.session_exists() {
                                    print_session_not_exist_error_msg();
                                    continue;
                                }
                                user.list_all_chat_rooms(&client).await?;
                            }
                            Some(Command::Quit) => {
                                // logout first
                                if user.session_exists() {
                                    user.logout(&client).await?;
                                }
        
                                // exit the app
                                print_msg("App is shutting down...");
                                print_msg("Bye!");
                                break;
                            }
                            None => {
                                print_warning_error_msg("Unknown command. Type 'help' to see available commands.")
                            }
                        }
                    }
                    "child" => {
                        if input == "exit" {
                            current_mode = "main";
                            print_msg("Exiting the chat interface...");
                            prompt = format!("{} >> ", user.get_user_name());
                        } else {
                            // todo messaging
                        }
                    }
                    _ => {
                        
                    }
                }
            }
        }
    }

    rl.save_history(history_file)?;
    Ok(())
}