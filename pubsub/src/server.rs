use crate::broker::Broker;
use crate::common::{ErrorMessage, SubscriptionMessage, UserMessage, PUBSUB_HOST_PORT};
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
                    broker: Broker::new().await,
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

    while let Some(Ok(msg)) = ws_receiver.next().await {
        match msg.as_text() {
            Some(text) => {
                if let Ok(sub_msg) = serde_json::from_str::<SubscriptionMessage>(&text) {
                    match broker.subscribe(&sub_msg, bcast_tx.clone()).await {
                        Ok(_) => (),
                        Err(e) => {
                            let err_message = ErrorMessage {
                                error: e,
                                message: format!(
                                    "Failed to subscribe to topic \"{}\".",
                                    &sub_msg.topic
                                ),
                            };
                            let msg: Message =
                                Message::text(serde_json::to_string(&err_message).unwrap());
                            let _ = ws_sender.send(msg).await;
                        }
                    }
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
                        broker.unsubscribe(&sub_msg).await;
                        break;
                    }
                    match serde_json::from_str::<UserMessage>(text) {
                        Ok(user_msg) => {
                            broker.publish(user_msg);
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
    sender_task.abort();
}
