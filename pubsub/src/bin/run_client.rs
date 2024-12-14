use pubsub::client::PubSubClient;
use tokio_websockets::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let username = String::from("user");
    let session_id = String::from("some_session_id");
    let topic = String::from("test");
    let mut pubsub_client: PubSubClient = PubSubClient::new(username, session_id).await?;
    pubsub_client.subscribe(topic).await?;
    pubsub_client.start().await
}
