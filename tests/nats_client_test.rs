
use anyhow::Result;

use nats_client_rs::{connect};

#[tokio::test]
async fn test_basic_client_connection() -> Result<()> {
    connect("localhost:4222".into()).await?;

    // block_on(async {
    //     loop {
    //         tokio::time::sleep(Duration::from_secs(2)).await
    //     }
    // });

    Ok(())
}
