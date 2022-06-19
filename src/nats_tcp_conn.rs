use std::pin::Pin;
use std::task::{Context, Poll};

use anyhow::Result;
use bytes::BytesMut;
use futures::{ready, Sink, Stream};
use tokio::io;
use tokio::io::AsyncWrite;

use crate::op::{ParserOp};
use crate::parser;

pub struct NatsTcpConn {
    stream: tokio::net::TcpStream,
    read_buffer: BytesMut,
    write_buffer: BytesMut,
    flushed: bool,
}

impl NatsTcpConn {
    pub fn new(stream: tokio::net::TcpStream) -> Self {
        Self {
            stream,
            read_buffer: BytesMut::with_capacity(8 * 1024),
            write_buffer: BytesMut::with_capacity(8 * 1024),
            flushed: true,
        }
    }

    fn decode(src: &mut BytesMut) -> Result<Option<ParserOp>> {
        if src.len() == 0 {
            return Ok(None);
        }

        parser::Parser::parse(String::from_utf8_lossy(src.as_ref()).into_owned().as_str()).and_then(|op| Ok(Some(op)))
    }
}

impl Stream for NatsTcpConn {
    type Item = ParserOp;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match NatsTcpConn::decode(&mut self.get_mut().read_buffer) {
            Ok(Some(op)) => Poll::Ready(Some(op)),
            Ok(None) => {
                Poll::Pending
            }
            Err(e) => {
                println!("could not decode: {}", e);
                Poll::Pending
            }

        }
    }
}

impl Sink<ParserOp> for NatsTcpConn {
    type Error = anyhow::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>> {
        if !self.flushed {
            match Pin::new(&mut self.get_mut().stream).poll_flush(cx)? {
                Poll::Ready(()) => Poll::Ready(Ok(())),
                Poll::Pending => Poll::Pending
            }
        } else {
            Poll::Ready(Ok(()))
        }
    }

    fn start_send(self: Pin<&mut Self>, item: ParserOp) -> std::result::Result<(), Self::Error> {
        let mut this = self.get_mut();

        this.write_buffer.extend(item.into_bytes()?);
        this.flushed = false;

        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>> {
        let mut this = self.get_mut();

        if this.flushed {
            return Poll::Ready(Ok(()));
        }

        let len = ready!(Pin::new(&mut this.stream).poll_write(cx, this.write_buffer.as_ref()))?;
        let wrote_all = len == this.write_buffer.len();
        this.flushed = true;
        this.write_buffer.clear();

        let res = if wrote_all {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "failed to write to socket").into())
        };

        Poll::Ready(res)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>> {
        ready!(self.poll_flush(cx))?;
        Poll::Ready(Ok(()))
    }
}
