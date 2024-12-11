use futures_util::sink::SinkExt;
use futures_util::stream::{SplitSink, SplitStream, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

const PUBSUB_HOST_PORT: &str = "127.0.0.1:8080";

#[derive(Deserialize, Serialize)]
struct SubscriptionMessage {
    topic: String,
    username: String,
}

#[derive(Deserialize, Serialize)]
struct UserMessage {
    topic: String,
    sender: String,
    content: String,
}

struct Subscriber {
    topic: String,
    username: String,
    sender: Sender<Message>,
}

#[derive(Clone)]
struct Broker {
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
    fn subscribe(&mut self, topic: String, username: String, sender: Sender<Message>) {
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

    async fn publish(&self, user_msg: UserMessage) {
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

pub struct PubSubServer {
    broker: Broker,
    listener: TcpListener,
}

impl PubSubServer {
    pub async fn new() -> Result<PubSubServer, std::io::Error> {
        match TcpListener::bind(PUBSUB_HOST_PORT).await {
            Ok(listener) => {
                println!("Listening on port 8080");
                Ok(PubSubServer {
                    broker: Broker::default(),
                    listener,
                })
            }
            Err(e) => {
                println!("Failed to bind TCP Listener. {e}");
                Err(e)
            }
        }
    }

    pub async fn start(&self) -> Result<(), std::io::Error> {
        loop {
            let (socket, _) = self.listener.accept().await?;
            match ServerBuilder::new().accept(socket).await {
                Ok(ws_stream) => {
                    let cloned_broker = self.broker.clone();
                    tokio::spawn(handle_connection(cloned_broker, ws_stream));
                }
                Err(e) => println!("Failed to handle new connection: {e}"),
            };
        }
    }
}

async fn handle_connection(mut broker: Broker, ws_stream: WebSocketStream<TcpStream>) {
    let (mut ws_sender, mut ws_receiver): (
        SplitSink<WebSocketStream<TcpStream>, Message>,
        SplitStream<WebSocketStream<TcpStream>>,
    ) = ws_stream.split();
    let (bcast_tx, mut bcast_rx): (Sender<Message>, Receiver<Message>) = channel(16);

    let _ = ws_sender
        .send(Message::text("Welcome to chat! Type a message".to_string()))
        .await;

    while let Some(Ok(msg)) = ws_receiver.next().await {
        match msg.as_text() {
            Some(text) => {
                if let Ok(sub_msg) = serde_json::from_str::<SubscriptionMessage>(&text) {
                    broker.subscribe(sub_msg.topic, sub_msg.username, bcast_tx.clone());
                    break;
                }
            }
            None => (),
        }
    }

    tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            match msg.as_text() {
                Some(text) => match serde_json::from_str::<UserMessage>(text) {
                    Ok(user_msg) => {
                        println!("Publishing received message...");
                        broker.publish(user_msg).await;
                    }
                    Err(e) => println!("Oops: {}", e),
                },
                None => (),
            }
        }
    });

    tokio::spawn(async move {
        while let Ok(message) = bcast_rx.recv().await {
            let _ = ws_sender.send(message).await;
        }
    });
}
