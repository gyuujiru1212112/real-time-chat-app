use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use http::Uri;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use tokio_websockets::tls::MaybeTlsStream;
use tokio_websockets::{ClientBuilder, Error, Message, WebSocketStream};

const PUBSUB_SERVER_ADDRESS: &str = "ws://127.0.0.1:8080";

#[derive(Deserialize, Serialize)]
struct SubscriptionMessage {
    topic: String,
    username: String,
}

struct PubSubClient {
    username: String,
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl PubSubClient {
    pub async fn new(username: String) -> Result<PubSubClient, Error> {
        let client_builder = ClientBuilder::from_uri(Uri::from_static(PUBSUB_SERVER_ADDRESS));
        match client_builder.connect().await {
            Ok((stream, _)) => Ok(PubSubClient { username, stream }),
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
        };
        let message = serde_json::to_string(&subscription_message).unwrap();
        self.stream.send(Message::text(message)).await
    }

    pub async fn start(&mut self) -> Result<(), Error> {
        let stdin = tokio::io::stdin();
        let mut stdin = BufReader::new(stdin).lines();

        loop {
            tokio::select! {
                incoming = self.stream.next() => {
                    match incoming {
                        Some(Ok(msg)) => {
                            if let Some(text) = msg.as_text() {
                                println!("From server: {}", text);
                            }
                        },
                        Some(Err(err)) => return Err(err.into()),
                        None => return Ok(()),
                    }
                }
                res = stdin.next_line() => {
                    match res {
                        Ok(None) => return Ok(()),
                        Ok(Some(line)) => self.stream.send(Message::text(line.to_string())).await?,
                        Err(err) => return Err(err.into()),
                    }
                }

            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut pubsub_client: PubSubClient = PubSubClient::new(String::from("test_user3")).await?;
    pubsub_client.subscribe(String::from("cats")).await?;
    pubsub_client.start().await
}
