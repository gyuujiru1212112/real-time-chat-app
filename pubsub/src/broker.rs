use crate::common::UserMessage;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast::Sender;
use tokio_websockets::Message;

struct Subscriber {
    topic: String,
    username: String,
    sender: Sender<Message>,
}

#[derive(Clone)]
pub struct Broker {
    subscribers: Arc<Mutex<HashMap<String, Vec<Subscriber>>>>,
}

impl Default for Broker {
    fn default() -> Broker {
        Broker {
            subscribers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Broker {
    pub fn subscribe(&mut self, topic: String, username: String, sender: Sender<Message>) {
        let mut subscribers = self.subscribers.lock().unwrap();
        let subscriber: Subscriber = Subscriber {
            topic: topic.clone(),
            username: username.clone(),
            sender,
        };
        subscribers
            .entry(topic.clone())
            .or_insert_with(Vec::new)
            .push(subscriber);
        println!("Subscribed user {} to topic {}", username, topic);
    }

    pub fn unsubscribe(&mut self, topic: String, username: String) {
        // TODO: Add error handling and make sure user is subscribed before unsubscribing.
        let mut subscribers = self.subscribers.lock().unwrap();
        let topic_subs = subscribers.get_mut(&topic).unwrap();
        let idx = topic_subs
            .iter()
            .position(|x| *x.username == username)
            .unwrap();
        topic_subs.remove(idx);
        println!("Unsubscribed user {} from topic {}", username, topic);
    }

    pub async fn publish(&self, user_msg: UserMessage) {
        let mut subscribers = self.subscribers.lock().unwrap();

        let msg: Message = Message::text(serde_json::to_string(&user_msg).unwrap());

        if let Some(topic_subscribers) = subscribers.get_mut(&user_msg.topic) {
            for subscriber in topic_subscribers.iter_mut() {
                // Send to all subscribers except for the sender itself.
                if subscriber.username != user_msg.sender {
                    let _ = subscriber.sender.send(msg.clone());
                }
            }
        }
    }
}
