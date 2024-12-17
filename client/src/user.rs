use reqwest::{header, Client, Url};
use rocket::serde::json::Value;
use rocket::serde::ser::StdError;
use rocket::serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
struct UserRequest {
    username: String,
    session_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserStatus {
    username: String,
    status: String,
}

#[derive(Deserialize, Serialize)]
pub struct ChatRoomInfo {
    username: String,
    session_id: String,
    room_name: String,
    members: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct PrivateChatRequest {
    username: String,
    session_id: String,
    recipient: String,
}

#[derive(Deserialize, Serialize)]
pub struct ChatRoomResponse {
    room_id: String,
    name: String,
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

    pub fn get_session_id(&self) -> String {
        self.session
            .as_ref()
            .map(|session| session.session_id.clone())
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
        let response = client.post(url).json(&signup_info).send().await?;
        if response.status().is_success() {
            print_msg("Signup successfully!");
        } else {
            print_warning_error_msg(&format!("Error: failed to signup: {}.", response.status()));
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
        let response = client.post(url).json(&login_info).send().await?;
        if response.status().is_success() {
            let json: Value = response.json().await.expect("Failed to parse JSON.");

            if let Some(session_id) = json.get("session_id").and_then(|v| v.as_str()) {
                // Create the session
                self.session = Some(Session::new(username, session_id));
                print_msg("Login successfully!");
                Ok(true)
            } else {
                print_warning_error_msg("Failed to retrieve session_id from JSON response.");
                Ok(false)
            }
        } else {
            self.error_response(&format!("Error: failed to login: {}.", response.status()))
        }
    }

    pub async fn logout(&mut self, client: &Client) -> Result<bool, Box<dyn StdError>> {
        let url = "http://localhost:8000/chatapp/user/logout"; // endpoint

        // Prepare the data
        let session = self.session.as_ref().unwrap();
        let logout_info = UserRequest {
            username: session.username.clone(),
            session_id: session.session_id.clone(),
        };

        // Send the POST request
        let response = client.post(url).json(&logout_info).send().await?;
        // Check if the response was successful
        if response.status().is_success() {
            self.session = None;
            print_msg("Log out successfully!");
            Ok(true)
        } else {
            self.error_response(&format!("Error: failed to logout: {}.", response.status()))
        }
    }

    pub async fn check_user_status(
        &mut self,
        client: &Client,
        user: String,
    ) -> Result<(), Box<dyn StdError>> {
        // endpoint
        let url = "http://localhost:8000/chatapp/user/status";
        let url = Url::parse_with_params(url, &[("username", &user)])?;

        // Get the current session
        let session = self.session.as_ref().ok_or("Session is not initialized")?;

        // Prepare headers
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "username",
            header::HeaderValue::from_str(&session.username)?,
        );
        headers.insert(
            "session_id",
            header::HeaderValue::from_str(&session.session_id)?,
        );

        // Send the GET request
        let response = client.get(url).headers(headers).send().await?;

        // Check the response status
        if response.status().is_success() {
            // Display the status
            let user_status = response.text().await?;
            print_msg(&format!("The status of user '{}' is {}", user, user_status));
        } else {
            print_warning_error_msg(&format!(
                "Error: failed to retrieve user's status: {}.",
                response.status()
            ));
        }

        Ok(())
    }

    pub async fn list_users(&mut self, client: &Client) -> Result<(), Box<dyn StdError>> {
        let url = "http://localhost:8000/chatapp/user/allusers"; // endpoint
        let session = self.session.as_ref().unwrap();

        // Send the GET request with headers
        let response = client
            .get(url)
            .header("username", &session.username)
            .header("session_id", &session.session_id)
            .send()
            .await?;
        if response.status().is_success() {
            let users: Vec<UserStatus> = response.json().await.expect("Failed to parse JSON");
            if users.is_empty() {
                print_warning_error_msg("Error: failed to get users.");
            } else {
                for user in users {
                    print_msg(&format!("user: {}, status: {}", user.username, user.status));
                }
            }
        } else {
            print_warning_error_msg(&format!(
                "Error: failed to retrieve users: {}.",
                response.status()
            ));
        }

        Ok(())
    }

    pub async fn create_private_chat(
        &mut self,
        client: &Client,
        user: String,
    ) -> Result<Option<String>, Box<dyn StdError>> {
        let session = self.session.as_ref().unwrap();
        if user == session.username {
            print_warning_error_msg("You are not allowed to create a private chat with yourself");
            return Ok(None);
        }

        let url = "http://localhost:8000/chatapp/chat/private-chat/create"; // endpoint

        let chat_info = PrivateChatRequest {
            username: session.username.clone(),
            session_id: session.session_id.clone(),
            recipient: user.clone(),
        };

        // Send the POST request
        let response = client.post(url).json(&chat_info).send().await?;

        // Check if the response was successful
        if response.status().is_success() {
            // get the chat_id
            let chat_id: String = response.json().await.expect("Failed to parse JSON");
            if chat_id.is_empty() {
                print_warning_error_msg("Error: failed to get the chat id.");
                Ok(None)
            } else {
                print_msg(&format!(
                    "You created a private chat with user '{}' successfully!",
                    user
                ));
                print_msg(&format!("Chat id is {}", chat_id));
                return Ok(Some(chat_id));
            }
        } else {
            print_warning_error_msg(&format!(
                "Error: failed to create a private chat with user '{}': {}.",
                user,
                response.status()
            ));
            Ok(None)
        }
    }

    pub async fn resume_private_chat(
        &self,
        client: &Client,
        user: String,
    ) -> Result<Option<String>, Box<dyn StdError>> {
        let session = self.session.as_ref().unwrap();
        if user == session.username {
            print_warning_error_msg("You are not allowed to create a private chat with yourself");
            return Ok(None);
        }

        let url = "http://localhost:8000/chatapp/chat/private-chat/resume"; // endpoint

        let chat_info = PrivateChatRequest {
            username: session.username.clone(),
            session_id: session.session_id.clone(),
            recipient: user.clone(),
        };
        // Send the POST request
        let response = client.post(url).json(&chat_info).send().await?;

        // Check if the response was successful
        if response.status().is_success() {
            // get the chat_id
            let chat_id: String = response.json().await.expect("Failed to parse JSON");
            if chat_id.is_empty() {
                print_warning_error_msg("Error: failed to get the chat id.");
                Ok(None)
            } else {
                print_msg(&format!(
                    "You resumed the private chat with user '{}' successfully!",
                    user
                ));
                print_msg(&format!("Chat id is {}", chat_id));
                return Ok(Some(chat_id));
            }
        } else {
            print_warning_error_msg(&format!(
                "Error: failed to resume the private chat with user '{}': {}.",
                user,
                response.status()
            ));
            Ok(None)
        }
    }

    pub async fn list_all_recipients(&self, client: &Client) -> Result<(), Box<dyn StdError>> {
        let url = "http://localhost:8000/chatapp/chat/private-chat/recipients"; // endpoint

        let session = self.session.as_ref().unwrap();
        // Send the GET request with headers
        let response = client
            .get(url)
            .header("username", &session.username)
            .header("session_id", &session.session_id)
            .send()
            .await?;
        if response.status().is_success() {
            let recipients: Vec<String> = response.json().await.expect("Failed to parse JSON");
            if recipients.is_empty() {
                print_msg("No private chat.");
            } else {
                let formatted = format!("[{}]", recipients.join(", "));
                print_msg(&formatted);
            }
        } else {
            print_warning_error_msg(&format!(
                "Error: failed to retrieve private chat recipients: {}.",
                response.status()
            ));
        }

        Ok(())
    }

    pub async fn create_chat_room(
        &mut self,
        client: &Client,
        room_name: String,
        members: &Vec<String>,
    ) -> Result<Option<String>, Box<dyn StdError>> {
        let session = self.session.as_ref().unwrap();

        let unique_members: HashSet<String> = members.iter().cloned().collect();
        if unique_members.is_empty() {
            print_warning_error_msg("You are not allowed to create a group chat without members");
            return Ok(None);
        }

        let chat_room_info = ChatRoomInfo {
            username: session.username.clone(),
            session_id: session.session_id.clone(),
            room_name: room_name.clone(),
            members: unique_members.into_iter().collect(),
        };

        let url = "http://localhost:8000/chatapp/chat/chat-room/create"; // endpoint

        // Send the POST request
        match client.post(url).json(&chat_room_info).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    let msg = format!("You created a chat room '{}' successfully!", room_name);
                    print_msg(&msg);
                    let chat_room_id: String = response.json().await.expect("Failed to parse JSON");
                    if chat_room_id.is_empty() {
                        return Ok(None);
                    } else {
                        print_msg(&format!("Chat room id is {}", chat_room_id));
                        return Ok(Some(chat_room_id));
                    }
                } else {
                    print_warning_error_msg(&format!(
                        "Failed to create chat room '{}': {}.",
                        room_name,
                        response.status()
                    ));
                    Ok(None)
                }
            }
            Err(error) => {
                print_warning_error_msg(&format!(
                    "Failed to create chat room '{}': {}.",
                    room_name, error
                ));
                Ok(None)
            }
        }
    }

    pub async fn join_chat_room(
        &self
    ) -> Result<(), Box<dyn StdError>> {
        Ok(())
    }

    pub async fn list_all_chat_rooms(&self, client: &Client) -> Result<(), Box<dyn StdError>> {
        let url = "http://localhost:8000/chatapp/chat/chat-room/all"; // endpoint

        let session = self.session.as_ref().unwrap();
        // Send the GET request with headers
        let response = client
            .get(url)
            .header("username", &session.username)
            .header("session_id", &session.session_id)
            .send()
            .await?;
        if response.status().is_success() {
            let chat_rooms: Vec<ChatRoomResponse> =
                response.json().await.expect("Failed to parse JSON");
            if chat_rooms.is_empty() {
                print_msg("No chat rooms.");
            } else {
                for room in chat_rooms {
                    print_msg(&format!("Name: {}, Room_id: {}", room.name, room.room_id));
                }
            }
        } else {
            print_warning_error_msg(&format!(
                "Error: failed to retrieve chat rooms: {}.",
                response.status()
            ));
        }

        Ok(())
    }

    fn error_response(&self, message: &str) -> Result<bool, Box<dyn StdError>> {
        print_warning_error_msg(message);
        Ok(false)
    }
}
