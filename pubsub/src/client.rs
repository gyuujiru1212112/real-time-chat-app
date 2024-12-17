use crate::common::{
    ErrorMessage, FetchHistoryMessage, SubscriptionAction, SubscriptionMessage, UserMessage,
    PUBSUB_SERVER_ADDRESS,
};
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use tokio_websockets::tls::MaybeTlsStream;
use tokio_websockets::{ClientBuilder, Error, Message, WebSocketStream};

pub struct PubSubClient {
    username: String,
    session_id: String,
    topic: Option<String>,
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl PubSubClient {
    pub async fn new(username: String, session_id: String) -> Result<PubSubClient, Error> {
        let client_builder = ClientBuilder::from_uri(Uri::from_static(PUBSUB_SERVER_ADDRESS));
        match client_builder.connect().await {
            Ok((stream, _)) => Ok(PubSubClient {
                username,
                session_id,
                topic: None,
                stream,
            }),
            Err(e) => {
                println!("Failed to connect to the pub-sub messaging server. {e}");
                Err(e)
            }
        }
    }

    pub async fn reconnect(&mut self) -> Result<(), Error> {
        let client_builder = ClientBuilder::from_uri(Uri::from_static(PUBSUB_SERVER_ADDRESS));
        // Make sure existing stream is closed first.
        let _ = self.stream.close().await;

        match client_builder.connect().await {
            Ok((stream, _)) => {
                self.stream = stream;
                return Ok(());
            }
            Err(e) => {
                println!("Failed to connect to the pub-sub messaging server. {e}");
                Err(e)
            }
        }
    }

    pub async fn subscribe(&mut self, topic: String) -> Result<(), Error> {
        let subscription_message: SubscriptionMessage = SubscriptionMessage {
            topic: topic.clone(),
            username: self.username.clone(),
            session_id: self.session_id.clone(),
            action: SubscriptionAction::Subscribe,
        };
        let message = Message::text(serde_json::to_string(&subscription_message).unwrap());
        match self.stream.send(message).await {
            Ok(()) => {
                self.topic = Some(topic);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub async fn unsubscribe(&mut self) -> Result<(), Error> {
        let subscription_message: SubscriptionMessage = SubscriptionMessage {
            topic: self.topic.clone().unwrap(),
            username: self.username.clone(),
            session_id: self.session_id.clone(),
            action: SubscriptionAction::Unsubscribe,
        };
        let message = Message::text(serde_json::to_string(&subscription_message).unwrap());
        match self.stream.send(message).await {
            Ok(()) => {
                self.topic = None;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub async fn fetch_history(&mut self) -> Result<(), Error> {
        let fetch_history_message: FetchHistoryMessage = FetchHistoryMessage {
            topic: self.topic.clone().unwrap(),
            username: self.username.clone(),
            num_messages: 10,
        };
        let message = Message::text(serde_json::to_string(&fetch_history_message).unwrap());
        match self.stream.send(message).await {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn start(&mut self) -> Result<(), Error> {
        let stdin = tokio::io::stdin();
        let mut stdin = BufReader::new(stdin).lines();

        // Consider using tokio::spawn instead of loop + tokio::select!
        loop {
            tokio::select! {
                incoming = self.stream.next() => {
                    match incoming {
                        Some(Ok(msg)) => {
                            match msg.as_text() {
                                Some(text) => {
                                    if let Ok(err_msg) = serde_json::from_str::<ErrorMessage>(&text) {
                                        println!("Error: {} -> {}", err_msg.error, err_msg.message);
                                        println!("Press enter key to exit.");
                                        self.stream.close().await?;
                                    } else if let Ok(user_msg) = serde_json::from_str::<UserMessage>(&text) {
                                        println!("{}: {}", user_msg.sender, user_msg.content);
                                    } else {
                                        println!("Unable to parse received message: {text}");
                                    }
                                }
                                None => (),
                            }
                        },
                        Some(Err(err)) => return Err(err.into()),
                        None => return Ok(()),
                    }
                }
                res = stdin.next_line() => {
                    match res {
                        Ok(None) => return Ok(()),
                        Ok(Some(line)) => {
                            // If there will be more commands, consider making an enum.
                            if line == ":help" {
                                println!("Chat Room Commands");
                                println!("------------------");
                                println!(":help --> Show chat command options");
                                println!(":exit --> Leave the chat");
                                println!(":history --> See the last 10 messages in the chat");
                                ()
                            } else if line == ":exit" {
                                println!("Leaving the chat...");
                                self.unsubscribe().await?;
                                self.stream.close().await?
                            } else if line == ":history" {
                                println!("Fetching chat history...");
                                self.fetch_history().await?
                            } else {
                                let user_message = self.create_user_message(line.to_string());
                                let message = Message::text(serde_json::to_string(&user_message).unwrap());
                                self.stream.send(message).await?
                            }
                        },
                        Err(err) => return Err(err.into()),
                    }
                }
            }
        }
    }

    fn create_user_message(&self, content: String) -> UserMessage {
        UserMessage {
            topic: self.topic.clone().unwrap(),
            sender: self.username.clone(),
            content,
        }
    }
}
