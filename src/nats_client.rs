use std::sync::Arc;
use crate::nats_options::NatsOptions;
use anyhow::Result;

const DEFAULT_HOST: &str = "localhost:4222";
const DEFAULT_URL_PROTOCOL: &str = "nats://";

pub struct NatsClient {
    url: String,
    options: NatsOptions,
}

impl Default for NatsClient {
    fn default() -> Self {
        NatsClient::new(DEFAULT_HOST.to_string(), NatsOptions::default())
    }
}

impl NatsClient {
    pub fn new(url: String, options: NatsOptions) -> Self {
        Self {
            url,
            options,
        }
    }

    pub async fn connect(self) -> Result<Arc<ConnectedNatsClient>> {
        let url = self.url;
        let conn = NatsConn::new(url).await?;
        let client = Arc::new(ConnectedNatsClient::new(conn, self.options));

        client.clone().start();

        Ok(client)
    }
}

pub struct ConnectedNatsClient {
    conn: NatsConn,
    options: NatsOptions,
}

impl ConnectedNatsClient {
    fn new(conn: NatsConn, options: NatsOptions) -> Self {
        Self {
            conn,
            options,
        }
    }

    async fn start(self: Arc<Self>) -> Result<()> {
        let nats_conn = self.clone();

        let _ = tokio::spawn(async move {
            while let Some(item) = nats_conn.conn.tcp_stream.next().await {
                nats_conn.process_nats_event(item).await;
            }
        });

        Ok(())
    }

    async fn process_nats_event(&self, event: ()) -> Result<()> {
        Ok(())
    }
}

pub struct NatsConn {
    tcp_stream: tokio::net::TcpStream,
}

impl NatsConn {
    async fn new(url: String) -> Result<Self> {
        let tcp_stream = tokio::net::TcpStream::connect(format!("{DEFAULT_URL_PROTOCOL}{url}")).await?;
        Ok(Self {
            tcp_stream
        })
    }

    async fn next(&self) -> Option<()> {
        Some(())
    }
}

