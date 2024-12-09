use std::collections::HashSet;

use clap::error;
use reqwest::{header, Client, Url};
use rocket::serde::json::Value;
use rocket::serde::ser::StdError;
use rocket::serde::{Deserialize, Serialize};

use crate::common::{print_msg, print_warning_error_msg};

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

#[derive(Deserialize, Serialize)]
pub struct ChatRoomInfo {
    name: String,
    users: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct PrivateChatInfo {
    user1: String,
    user2: String,
}

#[derive(Debug)]
pub struct Session {
    username: String,
    session_id: String,
}

impl Session {
    fn new(username: &str, session_id: &str) -> Self {
        Session {
            username: username.to_string(),
            session_id: session_id.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct User {
    session: Option<Session>,
}

impl User {
    pub fn new() -> Self {
        User { session: None }
    }

    pub fn get_user_name(&self) -> String {
        self.session
            .as_ref()
            .map(|session| session.username.clone())
            .unwrap_or_else(|| String::from(""))
    }

    pub fn session_exists(&mut self) -> bool {
        self.session.is_some()
    }

    pub async fn signup(
        &mut self,
        client: &Client,
        username: String,
        email: String,
        password: String,
    ) -> Result<(), Box<dyn StdError>> {
        let url = "http://localhost:8000/chatapp/user/signup"; // signup endpoint

        // Prepare the signup data
        let signup_info = SignupInfo {
            username,
            email,
            password,
        };

        // Send the POST request
        match client.post(url).json(&signup_info).send().await {
            Ok(response) => {
                let status = response.status();

                if status.is_success() {
                    print_msg("Signup successfully!");
                } else {
                    let msg = format!("Error: failed to signup: {}.", status);
                    print_warning_error_msg(&msg);
                }
            }
            Err(error) => {
                let msg = format!("Error: failed to signup: {}.", error);
                print_warning_error_msg(&msg);
            }
        }

        Ok(())
    }

    pub async fn login(
        &mut self,
        client: &Client,
        username: &str,
        password: String,
    ) -> Result<bool, Box<dyn StdError>> {
        let url = "http://localhost:8000/chatapp/user/login";
        let login_info = UserLogin {
            username: username.to_string(),
            password,
        };

        // Send the POST request
        match client.post(url).json(&login_info).send().await {
            Ok(response) => {
                let status = response.status();

                if status.is_success() {
                    let json: Value = response.json().await.expect("Failed to parse JSON.");

                    if let Some(session_id) = json.get("session_id").and_then(|v| v.as_str()) {
                        // Create the session
                        self.session = Some(Session::new(username, session_id));
                        print_msg("Login successfully!");
                        Ok(true)
                    } else {
                        print_warning_error_msg(
                            "Failed to retrieve session_id from JSON response.",
                        );
                        Ok(false)
                    }
                } else {
                    let msg = format!("Error: failed to login: {}.", status);
                    print_warning_error_msg(&msg);
                    Ok(false)
                }
            }
            Err(error) => {
                let msg = format!("Error: failed to login: {}.", error);
                print_warning_error_msg(&msg);
                Ok(false)
            }
        }
    }

    pub async fn logout(&mut self, client: &Client) -> Result<bool, Box<dyn StdError>> {
        let url = "http://localhost:8000/chatapp/user/logout"; // endpoint

        // Prepare the data
        let session = self.session.as_ref().unwrap();
        let logout_info = UserLogout {
            username: session.username.clone(),
            session_id: session.session_id.clone(),
        };

        // Send the POST request
        match client.post(url).json(&logout_info).send().await {
            Ok(response) => {
                let status = response.status();
                // Check if the response was successful
                if status.is_success() {
                    self.session = None;
                    println!("Log out successfully!");
                    Ok(true)
                } else {
                    let msg = format!("Error: failed to logout: {}.", status);
                    print_warning_error_msg(&msg);
                    Ok(false)
                }
            }
            Err(error) => {
                let msg = format!("Error: failed to logout: {}.", error.to_string());
                    print_warning_error_msg(&msg);
                    Ok(false)
            }
        }
    }

    pub async fn check_user_status(
        &mut self,
        client: &Client,
        user: String,
    ) -> Result<(), Box<dyn StdError>> {
        let url = "http://localhost:8000/chatapp/user/status";
        let url = Url::parse_with_params(url, &[("username", user.clone())])?;

        let session = self.session.as_ref().unwrap();
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "username",
            header::HeaderValue::from_str(&session.username)?,
        );
        headers.insert(
            "session_id",
            header::HeaderValue::from_str(&session.session_id)?,
        );

        let response = client
            .get(url)
            .headers(headers) // Add the headers
            .send()
            .await?;

        // Check if the response was successful
        if response.status().is_success() {
            // display the status
            let user_status = response.text().await.expect("Failed to get user status");
            println!("The status of user '{}' is {}", user, user_status);
        } else {
            let error_message = response.text().await?;
            println!(
                "Error: failed to retrieve user's status: {}.",
                error_message
            );
        }

        Ok(())
    }

    pub async fn list_active_users(&mut self, client: &Client) -> Result<(), Box<dyn StdError>> {
        let url = "http://localhost:8000/chatapp/user/allactive"; // endpoint
        let session = self.session.as_ref().unwrap();

        // Send the GET request with headers
        let response = client
            .get(url)
            .header("username", &session.username)
            .header("session_id", &session.session_id)
            .send()
            .await?;

        // Check if the response was successful
        if response.status().is_success() {
            // display the users
            let json: Value = response.json().await.expect("Failed to parse JSON");
            if let Some(array) = json.as_array() {
                let users: Vec<String> = array
                    .iter()
                    .filter_map(|item| item.as_str().map(|s| s.to_string()))
                    .collect();

                println!("The active users: {:?}", users);
            } else {
                eprintln!("Response is not an array of strings.");
            }
        } else {
            let error_message = response.text().await?;
            println!("Error: failed to retrieve active users: {}.", error_message);
        }
        Ok(())
    }

    pub async fn create_private_chat(
        &mut self,
        client: &Client,
        user: String,
    ) -> Result<bool, Box<dyn StdError>> {
        let session = self.session.as_ref().unwrap();
        if user == session.username {
            print_warning_error_msg("You are not allowed to create a private chat with yourself");
            return Ok(false);
        }

        let url = "http://localhost:8000/chatapp/chat/private-chat"; // endpoint

        let info = PrivateChatInfo {
            user1: session.username.clone(),
            user2: user.clone(),
        };
        // Send the POST request
        let response = client.post(url).json(&info).send().await?;

        // Check if the response was successful
        if response.status().is_success() {
            println!(
                "You created a private chat with user '{}' successfully!",
                user
            );
            Ok(true)
        } else {
            let error_message = response.text().await?;
            println!(
                "Error: failed to create a private chat with user '{}': {}.",
                user, error_message
            );
            Ok(false)
        }
    }

    async fn resume_private_chat(client: &Client, chatId: String) -> Result<(), Box<dyn StdError>> {
        Ok(())
    }

    pub async fn create_chat_room(
        &mut self,
        client: &Client,
        room_name: String,
        members: &Vec<String>,
    ) -> Result<bool, Box<dyn StdError>> {
        let session = self.session.as_ref().unwrap();

        // remove duplicate members using HashSet
        let mut members_to_pass: HashSet<String> = HashSet::new();
        members_to_pass.extend(members.clone());
        if members_to_pass.is_empty() {
            print_warning_error_msg("You are not allowed to create a group chat without members");
            return Ok(false);
        }

        members_to_pass.insert(session.username.clone());
        if members_to_pass.len() == 1 {
            print_warning_error_msg("You are not allowed to create a group chat with yourself");
            return Ok(false);
        }

        let info = ChatRoomInfo {
            name: room_name.clone(),
            users: members_to_pass.into_iter().collect(),
        };

        let url = "http://localhost:8000/chatapp/chat/chat-room"; // endpoint
                                                                  // Send the POST request
        let response = client.post(url).json(&info).send().await?;

        // Check if the response was successful
        if response.status().is_success() {
            println!("You created a chat room '{}' successfully!", room_name);
            Ok(true)
        } else {
            let error_message = response.text().await?;
            println!(
                "Error: failed to create a chat room '{}': {}.",
                room_name, error_message
            );
            Ok(false)
        }
    }

    async fn resume_chat_room(client: &Client, chatId: String) -> Result<(), Box<dyn StdError>> {
        Ok(())
    }
}
