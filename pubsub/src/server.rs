use crate::broker::Broker;
use crate::common::{SubscriptionMessage, UserMessage, PUBSUB_HOST_PORT};
use futures_util::sink::SinkExt;
use futures_util::stream::{SplitSink, SplitStream, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

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

    let receiver_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            match msg.as_text() {
                Some(text) => {
                    if let Ok(sub_msg) = serde_json::from_str::<SubscriptionMessage>(&text) {
                        broker.unsubscribe(sub_msg.topic, sub_msg.username);
                        break;
                    }
                    match serde_json::from_str::<UserMessage>(text) {
                        Ok(user_msg) => {
                            println!("Publishing received message...");
                            broker.publish(user_msg).await;
                        }
                        Err(e) => println!("Oops: {}", e),
                    }
                }
                None => (),
            }
        }
    });

    let sender_task = tokio::spawn(async move {
        while let Ok(message) = bcast_rx.recv().await {
            let _ = ws_sender.send(message).await;
        }
    });
    let _ = receiver_task.await;
    // println!("receiver_task finished");
    sender_task.abort();
    // println!("aborted sender_task");
}
