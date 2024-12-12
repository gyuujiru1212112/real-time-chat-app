use serde::{Deserialize, Serialize};
use std::fmt;

pub const PUBSUB_HOST_PORT: &str = "127.0.0.1:8080";
pub const PUBSUB_SERVER_ADDRESS: &str = "ws://127.0.0.1:8080";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum PubSubError {
    SubscriptionError,
}

impl fmt::Display for PubSubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ErrorMessage {
    pub error: PubSubError,
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub enum SubscriptionAction {
    Subscribe,
    Unsubscribe,
}

#[derive(Deserialize, Serialize)]
pub struct SubscriptionMessage {
    pub topic: String,
    pub username: String,
    pub session_id: String,
    pub action: SubscriptionAction,
}

#[derive(Deserialize, Serialize)]
pub struct UserMessage {
    pub topic: String,
    pub sender: String,
    pub content: String,
}
