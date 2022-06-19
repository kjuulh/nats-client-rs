use anyhow::Result;
use nats_client_rs::{connect};

#[tokio::test]
async fn test_basic_client_connection() -> Result<()> {
    connect("localhost:4222".into()).await?;

    Ok(())
}
