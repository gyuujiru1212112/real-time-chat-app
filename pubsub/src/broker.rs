use crate::common::{FetchHistoryMessage, PubSubError, SubscriptionMessage, UserMessage};
use crate::database::DbManager;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast::Sender;
use tokio_websockets::Message;

struct Subscriber {
    topic: Option<String>,
    sender: Sender<Message>,
}

#[derive(Clone)]
pub struct Broker {
    subscribers: Arc<Mutex<HashMap<String, Subscriber>>>,
    topics: Arc<Mutex<HashMap<String, Vec<String>>>>,
    db_manager: Arc<DbManager>,
}

impl Broker {
    pub async fn new() -> Broker {
        let db_manager = DbManager::new().await.unwrap();
        Broker {
            subscribers: Arc::new(Mutex::new(HashMap::new())),
            topics: Arc::new(Mutex::new(HashMap::new())),
            db_manager: Arc::new(db_manager),
        }
    }

    pub async fn subscribe(
        &mut self,
        sub_msg: &SubscriptionMessage,
        sender: Sender<Message>,
    ) -> Result<(), PubSubError> {
        match self
            .db_manager
            .is_session_id_valid(&sub_msg.username, &sub_msg.session_id)
            .await
        {
            true => {
                let mut subscribers = self.subscribers.lock().unwrap();
                let subscriber: Subscriber = Subscriber {
                    topic: Some(sub_msg.topic.clone()),
                    sender,
                };
                subscribers.insert(sub_msg.username.clone(), subscriber);

                let mut topics = self.topics.lock().unwrap();
                topics
                    .entry(sub_msg.topic.clone())
                    .or_insert_with(Vec::new)
                    .push(sub_msg.username.clone());

                println!(
                    "Subscribed user {} to topic {}",
                    sub_msg.username, sub_msg.topic
                );
                Ok(())
            }
            false => {
                println!(
                    "Failed to subscribe user \"{}\" to topic \"{}\": Invalid username + session_id",
                    sub_msg.username, sub_msg.topic
                );
                Err(PubSubError::SubscriptionError)
            }
        }
    }

    pub async fn unsubscribe(&mut self, sub_msg: &SubscriptionMessage) {
        match self
            .db_manager
            .is_session_id_valid(&sub_msg.username, &sub_msg.session_id)
            .await
        {
            true => {
                let mut topics = self.topics.lock().unwrap();
                let topic_subs = topics.get_mut(&sub_msg.topic).unwrap();
                let idx = topic_subs
                    .iter()
                    .position(|x| *x == sub_msg.username)
                    .unwrap();
                topic_subs.remove(idx);

                let mut subscribers = self.subscribers.lock().unwrap();
                let subscriber = subscribers.get_mut(&sub_msg.username).unwrap();
                subscriber.topic = None;
                println!(
                    "Unsubscribed user {} from topic {}",
                    sub_msg.username, sub_msg.topic
                );
            }
            false => {
                println!(
                    "Failed to Unsubscribe user {} from topic {}: Invalid username + session_id",
                    sub_msg.username, sub_msg.topic
                );
            }
        };
    }

    pub fn publish(&self, user_msg: UserMessage) {
        let subscribers = self.subscribers.lock().unwrap();
        let mut topics = self.topics.lock().unwrap();

        // Save the received message to db for chat history purposes.
        self.save_message(&user_msg);

        let msg: Message = Message::text(serde_json::to_string(&user_msg).unwrap());

        if let Some(topic_subs) = topics.get_mut(&user_msg.topic) {
            for subs_username in topic_subs.iter() {
                // Send to all subscribers except for the sender itself.
                if subs_username != &user_msg.sender {
                    let subscriber = subscribers.get(subs_username).unwrap();
                    let _ = subscriber.sender.send(msg.clone());
                }
            }
        }
    }

    fn save_message(&self, user_msg: &UserMessage) {
        let cloned_db_manager = self.db_manager.clone();
        let cloned_user_msg = user_msg.clone();
        tokio::spawn(async move {
            cloned_db_manager.save_message(&cloned_user_msg).await;
        });
    }

    pub async fn fetch_history(&self, hist_msg: &FetchHistoryMessage) {
        let subscribers = self.subscribers.lock().unwrap();
        let subscriber = subscribers.get(&hist_msg.username).unwrap();

        let cloned_sender = subscriber.sender.clone();
        let cloned_db_manager = self.db_manager.clone();
        let cloned_hist_msg = hist_msg.clone();

        tokio::spawn(async move {
            match cloned_db_manager
                .get_message_history(cloned_hist_msg.topic, cloned_hist_msg.num_messages)
                .await
            {
                Some(messages) => {
                    for hist_msg in messages.iter() {
                        let user_msg = UserMessage {
                            topic: hist_msg.chat_id.clone(),
                            sender: hist_msg.username.clone(),
                            content: hist_msg.message.clone(),
                        };
                        let msg: Message = Message::text(serde_json::to_string(&user_msg).unwrap());
                        let _ = cloned_sender.send(msg.clone());
                    }
                }
                None => {
                    println!("No message history returned");
                }
            }
        });
    }
}
