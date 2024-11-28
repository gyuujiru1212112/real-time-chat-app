use sqlx::{mysql::MySqlPool, Error, FromRow};

#[derive(FromRow)]
pub struct User {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub session_id: Option<String>,
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

    pub async fn set_user_session_id(&self, username: &String, session_id: &String) -> bool {
        let query = format!(
            "UPDATE user SET session_id = \"{}\" WHERE username = \"{}\";",
            session_id, username
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
