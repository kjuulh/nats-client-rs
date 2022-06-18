use nats_client_rs::error::NatsError;
use nats_client_rs::nats_client::NatsClient;
use anyhow::Result;

#[tokio::test]
async fn test_basic_client_connection() -> Result<()> {
    let nats_client = NatsClient::default().connect().await?;


    Ok(())
}
