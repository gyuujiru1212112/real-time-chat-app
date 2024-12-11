use pubsub::client::PubSubClient;
use tokio_websockets::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let username = String::from("user");
    let topic = String::from("test");
    let mut pubsub_client: PubSubClient = PubSubClient::new(username).await?;
    pubsub_client.subscribe(topic).await?;
    pubsub_client.start().await
}
