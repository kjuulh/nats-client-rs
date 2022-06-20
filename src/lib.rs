use anyhow::Result;

use futures::{SinkExt, StreamExt};


use crate::nats_tcp_conn::NatsTcpConn;

use crate::op::{NatsConnectOp, ParserOp};

pub mod parser;
mod op;
pub mod nats_tcp_conn;

async fn process_events(item: ParserOp) -> anyhow::Result<()> {
    match item {
        ParserOp::Connect(_) => {}
        ParserOp::Info(info) => {
            println!("INFO: {:?}", info)
        }
        ParserOp::Ping => {
            println!("PING")
        }
        ParserOp::Pong => {}
        ParserOp::Ok => {}
    }

    Ok(())
}

pub async fn connect(url: String) -> Result<()> {
    let stream = tokio::net::TcpStream::connect(url).await?;
    let (mut sink, mut conn) = NatsTcpConn::new(stream).split();
    let handle = tokio::spawn(async move {
        while let Some(item) = conn.next().await {
            println!("handling event: {:#?}", item);
            if let Err(e) = process_events(item).await {
                eprintln!("{}", e);
                break;
            }
        }
        println!("finished")
    });

    sink.send(ParserOp::Connect(NatsConnectOp {
        verbose: false,
        pedantic: false,
        tls_required: false,
        name: "some_name".to_string(),
        lang: "rust".to_string(),
        version: "1".to_string(),
        protocol: 1,
    })).await?;

    handle.await?;

    Ok(())
}
