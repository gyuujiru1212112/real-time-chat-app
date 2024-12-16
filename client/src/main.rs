mod commands;
mod common;
mod user;

use commands::{is_valid_email_addr, is_valid_password, is_valid_username, Command};
use common::{
    print_help_msg_after_login, print_help_msg_by_default, print_msg,
    print_session_exists_error_msg, print_session_not_exist_error_msg, print_warning_error_msg,
};
use pubsub::client::PubSubClient;
use reqwest::Client;
use std::sync::{Arc, Mutex};
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
    let mut pubsub_client: Option<Arc<Mutex<PubSubClient>>> = None;

    // get the command
    loop {
        match current_mode {
            "main" => {
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
                        match commands::parse_command(&input) {
                            Some(Command::Help) => {
                                if user.session_exists() {
                                    print_help_msg_after_login();
                                } else {
                                    print_help_msg_by_default();
                                }
                            }
                            Some(Command::Signup {
                                username,
                                email,
                                password,
                            }) => {
                                // check whether session exists
                                if user.session_exists() {
                                    print_warning_error_msg("You have already logged in!");
                                    print_warning_error_msg("Please log out first!");
                                    continue;
                                }

                                if !is_valid_username(&username)
                                    || !is_valid_email_addr(&email)
                                    || !is_valid_password(&password)
                                {
                                    continue;
                                }

                                user.signup(
                                    &client,
                                    username.to_string(),
                                    email.to_string(),
                                    password.to_string(),
                                )
                                .await?;
                            }
                            Some(Command::Login { username, password }) => {
                                // check whether session exists
                                if user.session_exists() {
                                    print_session_exists_error_msg();
                                    continue;
                                }

                                let res = user
                                    .login(&client, &username.to_string(), password.to_string())
                                    .await?;
                                if res {
                                    prompt = format!("{} >> ", user.get_user_name());

                                    // Create PubSub client on login.
                                    match &pubsub_client {
                                        Some(_) => (),
                                        None => {
                                            let ps_client = PubSubClient::new(
                                                user.get_user_name(),
                                                user.get_session_id(),
                                            )
                                            .await?;
                                            pubsub_client = Some(Arc::new(Mutex::new(ps_client)));
                                        }
                                    }
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
                                    // Remove PubSub client on logout.
                                    pubsub_client = None;
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
                                let res =
                                    user.create_private_chat(&client, with_user.clone()).await?;

                                match res {
                                    Some(chat_id) => {
                                        current_mode = "child";
                                        let enter_msg =
                                            format!("Entering private chat with {}...", with_user);
                                        print_msg(&enter_msg);
                                        match &pubsub_client {
                                            Some(ps_client) => {
                                                let _ = ps_client
                                                    .lock()
                                                    .unwrap()
                                                    .subscribe(chat_id)
                                                    .await;
                                            }
                                            None => {
                                                println!("Unable to join private chat. PubSub client is not initialized.");
                                            }
                                        }
                                    }
                                    None => {
                                        continue;
                                    }
                                }
                            }
                            Some(Command::ResumeChat { with_user }) => {
                                // check whether session exists
                                if !user.session_exists() {
                                    print_session_not_exist_error_msg();
                                    continue;
                                }
                                let res =
                                    user.resume_private_chat(&client, with_user.clone()).await?;
                                match res {
                                    Some(chat_id) => {
                                        current_mode = "child";
                                        let enter_msg =
                                            format!("Entering private chat with {}...", with_user);
                                        print_msg(&enter_msg);

                                        match &pubsub_client {
                                            Some(ps_client) => {
                                                let _ = ps_client
                                                    .lock()
                                                    .unwrap()
                                                    .subscribe(chat_id)
                                                    .await;
                                            }
                                            None => {
                                                println!("Unable to join chat room. PubSub client is not initialized.");
                                            }
                                        }
                                    }
                                    None => {
                                        continue;
                                    }
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
                                let res =
                                    user.create_chat_room(&client, name.clone(), &users).await?;

                                match res {
                                    Some(chat_room_id) => {
                                        current_mode = "child";
                                        let enter_msg = format!("Entering chat room {}...", name);
                                        print_msg(&enter_msg);
                                        match &pubsub_client {
                                            Some(ps_client) => {
                                                let _ = ps_client
                                                    .lock()
                                                    .unwrap()
                                                    .subscribe(chat_room_id)
                                                    .await;
                                            }
                                            None => {
                                                println!("Unable to join chat room. PubSub client is not initialized.");
                                            }
                                        }
                                    }
                                    None => {
                                        continue;
                                    }
                                }
                            }
                            Some(Command::JoinChatRoom { chat_id }) => {
                                // check whether session exists
                                if !user.session_exists() {
                                    print_session_not_exist_error_msg();
                                    continue;
                                }
                                let res = user.join_chat_room().await;
                                match res {
                                    Ok(()) => {
                                        current_mode = "child";
                                        let enter_msg =
                                            format!("Entering chat room with id {}...", chat_id);
                                        print_msg(&enter_msg);

                                        match &pubsub_client {
                                            Some(ps_client) => {
                                                let _ = ps_client
                                                    .lock()
                                                    .unwrap()
                                                    .subscribe(chat_id)
                                                    .await;
                                            }
                                            None => {
                                                println!("Unable to join chat room. PubSub client is not initialized.");
                                            }
                                        }
                                    }
                                    Err(_) => {
                                        continue;
                                    }
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
                            None => print_warning_error_msg(
                                "Unknown command. Type 'help' to see available commands.",
                            ),
                        }
                    }
                }
            }
            "child" => match &pubsub_client {
                Some(ps_client) => {
                    let _ = ps_client.lock().unwrap().start().await;
                    // This is kind of hacky but to exit the chat, the stream is closed so
                    // reconnect the stream so that the user can reuse the same pubsub client.
                    // Will improve this later if there is time.
                    let _ = ps_client.lock().unwrap().reconnect().await;
                    println!("Exited the chat");
                    current_mode = "main";
                    prompt = format!("{} >> ", user.get_user_name());
                    continue;
                }
                None => (),
            },
            _ => {}
        }
    }

    rl.save_history(history_file)?;
    Ok(())
}
