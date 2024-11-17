use std::io::{self, Write};
use reqwest::Client;
use rocket::serde::ser::StdError;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SignupInfo {
    username: String,
    email: String,
    password: String,
}


#[derive(Deserialize, Serialize)]
struct UserLogin {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
struct UserLogout {
    username: String,
    session_id: String,
}

async fn signup(client: &Client, username: String, email: String, password: String) -> Result<(), Box<dyn StdError>>
{
    let url = "http://localhost:8000/chatapp/user/signup"; // signup endpoint

    // Prepare the signup data
    let signup_info = SignupInfo { username, email, password };

    // Send the POST request
    let response = client
        .post(url)
        .json(&signup_info)
        .send()
        .await?;

    // Check if the response was successful
    if response.status().is_success() {
        println!("Signup successfully!");
    } else {
        let error_message = response.text().await?;
        println!("Error: failed to signup user: {}", error_message);
    }

    Ok(())
}

async fn login(client: &Client, username: String, password: String) -> Result<(), Box<dyn StdError>>
{
    let url = "http://localhost:8000/chatapp/user/login";
    let login_info = UserLogin { username, password };

    // Send the POST request
    let response = client
        .post(url)
        .json(&login_info)
        .send()
        .await?;

    // Check if the response was successful
    if response.status().is_success() {
        // todo get the session id
        println!("login successfully!");
    } else {
        let error_message = response.text().await?;
        println!("Error: failed to login user: {}", error_message);
    }

    Ok(())
}

async fn logout(client: &Client) -> Result<(), Box<dyn StdError>>
{
    // todo session id
    Ok(())
}

async fn check_user_status(client: &Client, user: String) -> Result<(), Box<dyn StdError>>
{
    Ok(())
}

async fn list_active_users(client: &Client) -> Result<(), Box<dyn StdError>>
{
    Ok(())
}

async fn create_private_chat(client: &Client, user: String) -> Result<(), Box<dyn StdError>>
{
    // todo enter a child CLI interface
    Ok(())
}

async fn resume_private_chat(client: &Client, chatId: String) -> Result<(), Box<dyn StdError>>
{
    // todo enter a child CLI interface
    Ok(())
}

async fn create_chat_room(client: &Client, room_name: String) -> Result<(), Box<dyn StdError>>
{
    // todo enter a child CLI interface
    Ok(())
}

async fn resume_chat_room(client: &Client, chatId: String) -> Result<(), Box<dyn StdError>>
{
    // todo enter a child CLI interface
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the real time chat app!");
    println!("Type 'help' to see available commands.");
    // todo launch the server
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
                signup(&client, username.to_string(), email.to_string(), password.to_string()).await?;
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