use sqlx::mysql::MySqlPool;

pub struct DbManager {
    pub db_url: String,
    conn_pool: Option<MySqlPool>,
}

impl DbManager {
    pub fn new(db_url: String) -> DbManager {
        DbManager {
            db_url,
            conn_pool: None,
        }
    }

    pub async fn connect(&mut self) {
        match MySqlPool::connect(&self.db_url).await {
            Ok(pool) => {
                self.conn_pool = Some(pool);
                println!("Connected to db!");
            }
            Err(e) => {
                println!("Failed to connect to database with error: {}", e)
            }
        }
    }
}
