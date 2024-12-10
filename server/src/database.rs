use sqlx::{mysql::MySqlPool, Row, Error, FromRow};

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

    pub async fn insert_private_chat(&self, user1: &str, user2: &str) -> bool
    {
        let query = format!(
            "INSERT INTO private_chat (user1, user2) VALUES (\"{}\", \"{}\");",
            user1, user2
        );

        let result = sqlx::query(&query).execute(&self.conn_pool).await;
        match result {
            Ok(_) => true,
            Err(e) => {
                println!("Error inserting private_chat between users '{}' and '{}' : {}", user1, user2, e.to_string());
                false
            }
        }
    }

    pub async fn get_all_chat_recipients(&self, username: &str) -> Option<Vec<String>>
    {
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
            .fetch_all(&self.conn_pool).await;

        match result {
            Ok(partner_rows) => {
                let partners = partner_rows.iter()
                    .map(|row| row.get("partner")).collect();

                Some(partners)
            }

            Err(e) => {
                println!("Error querying private_chat table for user '{}': {}", username, e);
                None
            }
        }

    }

    pub async fn insert_chat_room(&self, name: &str, users: &Vec<String>) -> bool {
        if users.is_empty() {
            println!("Cannot create the chat room '{}' without members", name);
            return false
        }
        // insert the chat room
        let query = 
            "INSERT INTO chat_room (name) VALUES (?)";
        let room_id = match sqlx::query(&query)
            .bind(name)
            .execute(&self.conn_pool)
            .await
            {
                Ok(result) => result.last_insert_id(),
                Err(e) => {
                    println!("Error inserting chat room '{}' : {}", name, e.to_string());
                    return false
                }
            };

        // insert the members
        let insert_user_query = "INSERT INTO room_member (room_id, username) VALUES (?, ?)";
        for user in users
        {
            if let Err(e) = sqlx::query(&insert_user_query)
                .bind(room_id)
                .bind(user)
                .execute(&self.conn_pool)
                .await
                {
                    println!("Failed to insert user '{}' into room {}: {}", user, name, e.to_string());
                };
        }

        true
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
        let query = "SELECT id, name FROM chat_room";
        let result= sqlx::query_as::<_, ChatRoom>(&query).fetch_all(&self.conn_pool)
            .await;
        match result {
            Ok(chat_rooms) => {
                if chat_rooms.is_empty() {
                    None
                } else {
                    let mut names = Vec::new();
                    for room in chat_rooms
                    {
                        names.push((room.id.to_string(), room.name));
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

    // pub async fn execute(&self, query: &str) -> bool {
    //     let result = sqlx::query(query).execute(&self.conn_pool).await;
    //     match result {
    //         Ok(query_result) => true,
    //         Err(e) => {
    //             println!("Error executing query {} : {}", query, e.to_string());
    //             false
    //         }
    //     }
    // }
}
