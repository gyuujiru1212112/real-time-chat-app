use sqlx::{mysql::MySqlPool, Error, FromRow, Row};
use uuid::Uuid;

#[derive(FromRow)]
pub struct User {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct ChatRoom {
    chat_room_id: String,
    name: String,
}

pub struct DbManager {
    conn_pool: MySqlPool,
}

impl DbManager {
    pub async fn new(db_url: String) -> Result<DbManager, Error> {
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

    pub async fn insert_private_chat(&self, user1: &str, user2: &str) -> Option<String> {
        // (user1, user2) should be the same as (user2, user1)
        let (user1, user2) = if user1 < user2 {
            (user1, user2)
        } else {
            (user2, user1)
        };
        let query = format!(
            "INSERT INTO private_chat (user1, user2) VALUES (\"{}\", \"{}\");",
            user1, user2
        );

        let result = sqlx::query(&query).execute(&self.conn_pool).await;
        match result {
            Ok(_) => {
                // update chat_id
                let chat_id = Uuid::new_v4().to_string();
                println!("Chat id created");
                if self.set_chat_id(user1, user2, &chat_id).await {
                    Some(chat_id)
                } else {
                    None
                }
            }
            Err(e) => {
                println!(
                    "Error inserting private_chat between users '{}' and '{}' : {}",
                    user1,
                    user2,
                    e.to_string()
                );
                None
            }
        }
    }

    pub async fn get_all_chat_recipients(&self, username: &str) -> Option<Vec<String>> {
        let query = r#"
            SELECT user1 AS partner
            FROM private_chat
            WHERE user2 = ?
            UNION
            SELECT user2 AS partner
            FROM private_chat
            WHERE user1 = ?;
            "#;

        let result = sqlx::query(&query)
            .bind(username)
            .bind(username)
            .fetch_all(&self.conn_pool)
            .await;

        match result {
            Ok(partner_rows) => {
                let partners = partner_rows.iter().map(|row| row.get("partner")).collect();

                Some(partners)
            }

            Err(e) => {
                println!(
                    "Error querying private_chat table for user '{}': {}",
                    username, e
                );
                None
            }
        }
    }

    pub async fn get_chat_id(&self, user1: &str, user2: &str) -> Option<String> {
        // (user1, user2) should be the same as (user2, user1)
        let (user1, user2) = if user1 < user2 {
            (user1, user2)
        } else {
            (user2, user1)
        };

        let query = "SELECT chat_id FROM private_chat WHERE user1 = ? AND user2 = ?";
        let result = sqlx::query(&query)
            .bind(user1)
            .bind(user2)
            .fetch_optional(&self.conn_pool)
            .await;
        match result {
            Ok(row) => row.map(|r| r.get::<String, _>("chat_id")),
            Err(e) => {
                println!("Failed to retrieve the chat_id: {}", e.to_string());
                None
            }
        }
    }

    pub async fn insert_chat_room(&self, name: &str) -> Option<String> {
        // insert the chat room
        let query = "INSERT INTO chat_room (name) VALUES (?)";
        let id = match sqlx::query(&query)
            .bind(name)
            .execute(&self.conn_pool)
            .await
        {
            Ok(result) => result.last_insert_id(),
            Err(e) => {
                println!("Error inserting chat room '{}' : {}", name, e.to_string());
                return None;
            }
        };

        // update chat_room_id
        let chat_room_id = Uuid::new_v4().to_string();
        let res = self.set_chat_room_id(id, name, &chat_room_id).await;
        if res {
            Some(chat_room_id)
        } else {
            None
        }
    }

    pub async fn insert_user(&self, username: &String, email: &String, password: &String) -> bool {
        let query = format!(
            "INSERT INTO user (username, email, password) VALUES (\"{}\", \"{}\", \"{}\");",
            username, email, password
        );

        let result = sqlx::query(&query).execute(&self.conn_pool).await;
        match result {
            Ok(_) => true,
            Err(e) => {
                println!("Error inserting user {} : {}", username, e.to_string());
                false
            }
        }
    }

    pub async fn set_user_session_id(
        &self,
        username: &String,
        session_id: Option<&String>,
    ) -> bool {
        let session_id_value = match session_id {
            Some(id) => format!("\"{}\"", id),
            None => String::from("null"),
        };
        let query = format!(
            "UPDATE user SET session_id = {} WHERE username = \"{}\";",
            session_id_value, username
        );
        let result = sqlx::query(&query).execute(&self.conn_pool).await;
        match result {
            Ok(_) => true,
            Err(e) => {
                println!("Error inserting user {} : {}", username, e.to_string());
                false
            }
        }
    }

    async fn set_chat_id(&self, user1: &str, user2: &str, chat_id: &str) -> bool {
        let query = "UPDATE private_chat SET chat_id = ? WHERE user1 = ? AND user2 = ?";
        let result = sqlx::query(&query)
            .bind(chat_id)
            .bind(user1)
            .bind(user2)
            .execute(&self.conn_pool)
            .await;
        match result {
            Ok(_) => true,
            Err(e) => {
                println!(
                    "Error inserting chat id for chat between '{}' and '{}' : {}",
                    user1,
                    user2,
                    e.to_string()
                );
                false
            }
        }
    }

    async fn set_chat_room_id(&self, id: u64, room_name: &str, chat_id: &str) -> bool {
        let query = "UPDATE chat_room SET chat_room_id = ? WHERE id = ? AND name = ?";
        let result = sqlx::query(&query)
            .bind(chat_id)
            .bind(id)
            .bind(room_name)
            .execute(&self.conn_pool)
            .await;
        match result {
            Ok(_) => true,
            Err(e) => {
                println!(
                    "Error inserting chat id for chat room '{}': {}",
                    room_name,
                    e.to_string()
                );
                false
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

    pub async fn get_all_users(&self) -> Option<Vec<User>> {
        let query = "SELECT * FROM user;";
        let result = sqlx::query_as::<_, User>(&query)
            .fetch_all(&self.conn_pool)
            .await;
        match result {
            Ok(users) => Some(users),
            Err(e) => {
                println!("Error querying user table: {}", e);
                None
            }
        }
    }

    pub async fn get_all_chat_rooms(&self) -> Option<Vec<(String, String)>> {
        let query = "SELECT chat_room_id, name FROM chat_room";
        let result = sqlx::query_as::<_, ChatRoom>(&query)
            .fetch_all(&self.conn_pool)
            .await;
        match result {
            Ok(chat_rooms) => {
                if chat_rooms.is_empty() {
                    None
                } else {
                    let mut names = Vec::new();
                    for room in chat_rooms {
                        names.push((room.chat_room_id, room.name));
                    }
                    Some(names)
                }
            }
            Err(e) => {
                println!("Error querying chat_room table: {}", e);
                None
            }
        }
    }
}
