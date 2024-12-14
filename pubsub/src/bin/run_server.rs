use pubsub::server::PubSubServer;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let pubsub_server = PubSubServer::new().await?;
    pubsub_server.start().await
}
