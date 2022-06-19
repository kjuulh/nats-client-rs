


use anyhow::Result;

use futures::{SinkExt, StreamExt};


use crate::nats_tcp_conn::NatsTcpConn;

use crate::op::{NatsConnectOp, ParserOp};

pub mod parser;
mod op;
pub mod nats_tcp_conn;


pub async fn connect(url: String) -> Result<()> {
    let stream = tokio::net::TcpStream::connect(url).await?;
    let (mut sink, mut conn) = NatsTcpConn::new(stream).split();
     let _ = tokio::spawn(async move {
         while let Some(item) = conn.next().await {
             println!("{:#?}", item)
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

    Ok(())
}
