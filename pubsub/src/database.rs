use crate::common::UserMessage;
use sqlx::{mysql::MySqlPool, Error, FromRow};
use std::env;

#[derive(FromRow)]
pub struct User {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub session_id: Option<String>,
}

#[derive(FromRow)]
pub struct ChatMessage {
    pub chat_id: String,
    pub username: String,
    pub message: String,
}

pub struct DbManager {
    conn_pool: MySqlPool,
}

impl DbManager {
    pub async fn new() -> Result<DbManager, Error> {
        let db_url: String = env::var("MYSQL_URL").unwrap_or(String::from(
            "mysql://chatserver:ServerPass123@localhost:3306/chatapp",
        ));

        match MySqlPool::connect(&db_url).await {
            Ok(pool) => {
                println!("Connected to db!");
                Ok(DbManager { conn_pool: pool })
            }
            Err(e) => {
                println!("Failed to connect to database with error: {}", e);
                Err(e)
            }
        }
    }

    pub async fn get_user(&self, username: &String) -> Option<User> {
        let query = format!("SELECT * FROM user WHERE username = \"{}\";", username);
        let result = sqlx::query_as::<_, User>(&query)
            .fetch_one(&self.conn_pool)
            .await;
        match result {
            Ok(user) => Some(user),
            Err(e) => {
                println!("Error querying user table for {} : {}", username, e);
                None
            }
        }
    }

    pub async fn is_session_id_valid(&self, username: &String, given_session_id: &String) -> bool {
        let mut is_valid: bool = false;
        match self.get_user(username).await {
            Some(user) => match user.session_id {
                Some(session_id) => {
                    if session_id == *given_session_id {
                        println!("Valid username and session_id pair.");
                        is_valid = true;
                    } else {
                        println!(
                            "Given session_id does not match with expected value for user \'{}\'.",
                            username
                        );
                    }
                }
                None => println!("No current session_id for user \'{}\'.", username),
            },
            None => println!("Invalid username. Could not find user \'{}\'.", username),
        }
        is_valid
    }

    pub async fn save_message(&self, user_msg: &UserMessage) {
        let query = format!(
            "INSERT INTO chat_message (chat_id, username, message) VALUES (\"{}\", \"{}\", \"{}\");",
            user_msg.topic, user_msg.sender, user_msg.content
        );

        let result = sqlx::query(&query).execute(&self.conn_pool).await;
        match result {
            Ok(_) => (),
            Err(e) => {
                println!(
                    "Error inserting message from user {} in chat {} : {}",
                    user_msg.sender,
                    user_msg.topic,
                    e.to_string()
                );
            }
        }
    }

    pub async fn get_message_history(
        &self,
        topic: String,
        num_messages: usize,
    ) -> Option<Vec<ChatMessage>> {
        // "SELECT * FROM chat_message WHERE chat_id = \"{}\" LIMIT {};",
        // "SELECT * FROM (SELECT * FROM chat_message WHERE chat_id = \"{}\" ORDER BY id DESC LIMIT {}) AS sub ORDER BY id ASC;"
        let query = format!(
            "SELECT * FROM (SELECT * FROM chat_message WHERE chat_id = \"{}\" ORDER BY id DESC LIMIT {}) AS sub ORDER BY id ASC;",
            &topic, num_messages
        );

        let result = sqlx::query_as::<_, ChatMessage>(&query)
            .fetch_all(&self.conn_pool)
            .await;
        match result {
            Ok(messages) => {
                println!("number of history messages returned: {}", messages.len());
                Some(messages)
            }
            Err(e) => {
                println!("Error querying chat_message table for {} : {}", topic, e);
                None
            }
        }
    }
}
