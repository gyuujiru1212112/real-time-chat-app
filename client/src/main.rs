mod commands;

use std::io::{self, Write};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the real time chat app!");
    println!("Type 'help' to see available commands.");
    // client
    let client = Client::new();

    // get the command
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        match input {
            "help" => {
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
            command if input.starts_with("signup") => {
                // hard-coded for testing

                // todo validate input
                let username = "yiduo";
                let email = "ydjing121@gmail.com";
                let password = "!Abcd";
                commands::signup(&client, username.to_string(), email.to_string(), password.to_string()).await?;
            }
            command if input.starts_with("login") => {

                // login successfully
                // todo setup websocket
            }
            command if input.starts_with("logout") => {

            }
            
            "exit" => {
                // exit the app
                println!("App is shutting down...");
                break;
            }
            _ => {
                println!("Unknown command. Type 'help' to see available commands.")
            }
        }
    }
    Ok(())
}