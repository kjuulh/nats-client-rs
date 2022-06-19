pub mod parser;
mod op;

use std::pin::Pin;
use std::task::{Context, Poll};

use anyhow::Result;
use bytes::{Buf, BytesMut};
use futures::Stream;
use futures::stream::SelectNextSome;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::sequence::delimited;
use tokio::io::AsyncRead;

enum NatsOp {}

struct NatsTcpConn {
    stream: tokio::net::TcpStream,
    read_buffer: BytesMut,
}

impl NatsTcpConn {
    fn new(stream: tokio::net::TcpStream) -> Self {
        Self { stream, read_buffer: BytesMut::with_capacity(8 * 1024) }
    }

    // fn decode(src: &mut BytesMut) -> Result<NatsOp> {
    //     Parser::decode(src)
    // }
}

// impl Stream for NatsTcpConn {
//     type Item = NatsOp;
//
//     fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
//         //NatsTcpConn::decode(&mut self.read_buffer)
//     }
// }

pub async fn connect(url: String) -> Result<()> {
    let stream = tokio::net::TcpStream::connect(url).await?;
    let _ = NatsTcpConn::new(stream);

    Ok(())
}
