use reqwest::Client;
use rocket::serde::json::Value;
use rocket::serde::{json, Deserialize, Serialize};
use rocket::serde::ser::StdError;

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

#[derive(Debug)]
pub struct Session
{
    username: String,
    session_id: String
}

impl  Session  {
    fn new(username: &str, session_id: &str) -> Self
    {
        Session { 
            username: username.to_string(), 
            session_id: session_id.to_string()
        }
    }
}

#[derive(Debug)]
pub struct User
{
    session: Option<Session>
}

impl User {
    pub fn new() -> Self {
        User {
            session: None,
        }
    }

    pub async fn signup(&mut self, client: &Client, username: String, email: String, password: String) -> Result<(), Box<dyn StdError>>
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
            println!("Error: failed to signup user: {}.", error_message);
        }

        Ok(())
    }

    pub async fn login(&mut self, client: &Client, username: &str, password: String) -> Result<(), Box<dyn StdError>>
    {
        // check whether session exists
        if self.session.is_some() {
            println!("You have already logged in!");
            println!("Please log out first before logging into another account!");
            return Ok(());
        }

        let url = "http://localhost:8000/chatapp/user/login";
        let login_info = UserLogin { 
            username: username.to_string(), 
            password
        };

        // Send the POST request
        let response = client
            .post(url)
            .json(&login_info)
            .send()
            .await?;

        // Check if the response was successful
        if response.status().is_success() {
            let json : Value = response.json().await.expect("Failed to parse JSON");

            // get the session id
            if let Some(session_id) = json.get("session_id") {
                // create the session
                self.session = Some(Session::new(username, &session_id.to_string()));
                println!("login successfully!");
            }

        } else {
            let error_message = response.text().await?;
            println!("Error: failed to login user: {}.", error_message);
        }

        Ok(())
    }

    pub async fn logout(&mut self, client: &Client) -> Result<(), Box<dyn StdError>>
    {
        // todo session id
        Ok(())
    }

    async fn check_user_status(&mut self, client: &Client, user: String) -> Result<(), Box<dyn StdError>>
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
}