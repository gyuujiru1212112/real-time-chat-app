use sqlx::{mysql::MySqlPool, Error, FromRow};
use std::env;

#[derive(FromRow)]
pub struct User {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct ChatRoom {
    id: i64,
    name: String,
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
}
