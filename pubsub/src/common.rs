use serde::{Deserialize, Serialize};

pub const PUBSUB_HOST_PORT: &str = "127.0.0.1:8080";
pub const PUBSUB_SERVER_ADDRESS: &str = "ws://127.0.0.1:8080";

#[derive(Deserialize, Serialize)]
pub struct SubscriptionMessage {
    pub topic: String,
    pub username: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserMessage {
    pub topic: String,
    pub sender: String,
    pub content: String,
}
