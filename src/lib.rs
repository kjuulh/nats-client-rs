use anyhow::Result;
use std::sync::Arc;

use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

use crate::nats_tcp_conn::NatsTcpConn;

use crate::op::{NatsConnectOp, ParserOp};

pub mod nats_tcp_conn;
mod op;
pub mod parser;

async fn process_events(
    conn: Arc<Mutex<SplitSink<NatsTcpConn, ParserOp>>>,
    item: ParserOp,
) -> anyhow::Result<()> {
    match item {
        ParserOp::Connect(_) => {}
        ParserOp::Info(info) => {
            println!("INFO: {:?}", info)
        }
        ParserOp::Ping => {
            let mut conn = conn.lock().await;
            conn.send(ParserOp::Pong).await?;
            println!("PING")
        }
        ParserOp::Pong => {}
        ParserOp::Ok => {}
    }

    Ok(())
}

pub async fn connect(url: String) -> Result<()> {
    let stream = tokio::net::TcpStream::connect(url).await?;
    let (sink, mut conn) = NatsTcpConn::new(stream).split();
    let arc_sink = Arc::new(Mutex::new(sink));
    let sending_sink = arc_sink.clone();
    let handle = tokio::spawn(async move {
        while let Some(item) = conn.next().await {
            println!("handling event: {:#?}", item);
            if let Err(e) = process_events(sending_sink.clone(), item).await {
                eprintln!("{}", e);
                break;
            }
            io::stdout().flush().await.unwrap();
        }
        println!("finished");
    });

    arc_sink
        .lock()
        .await
        .send(ParserOp::Connect(NatsConnectOp {
            verbose: false,
            pedantic: false,
            tls_required: false,
            name: "some_name".to_string(),
            lang: "rust".to_string(),
            version: "1".to_string(),
            protocol: 1,
        }))
        .await?;

    handle.await?;

    Ok(())
}
