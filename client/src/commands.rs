use rocket::serde::ser::StdError;
use rocket::serde::{Deserialize, Serialize};
use validator::ValidateEmail;
use regex::Regex;
use reqwest::Client;

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

fn is_valid_email_addr(email: &str) -> bool
{
    email.validate_email()
}

fn is_valid_username(username: &str) -> bool
{
    let re = Regex::new(r"^(?=.{8,20}$)(?![_.])(?!.*[_.]{2})[a-zA-Z0-9._]+(?<![_.])$").unwrap();
    re.is_match(username)
}

fn is_valid_password(password: &str) -> bool
{
    let re = Regex::new(r"/^(?=.*[0-9])(?=.*[!@#$%^&*])[a-zA-Z0-9!@#$%^&*]{6,16}$/").unwrap();
    re.is_match(password)
}

pub async fn signup(client: &Client, username: String, email: String, password: String) -> Result<(), Box<dyn StdError>>
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