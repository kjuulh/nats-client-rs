use crate::op::ParserOp;
use crate::parser;
use anyhow::Result;
use bytes::BytesMut;
use futures::{ready, Sink, Stream};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

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
        if src.is_empty() {
            return Ok(None);
        }

        let cmd = String::from_utf8_lossy(src.as_ref()).into_owned();
        match parser::Parser::parse(cmd.as_str()).map(Some) {
            Ok(op) => {
                println!("parsing ok {:#?}", op);
                Ok(op)
            }
            Err(e) => {
                eprintln!("parsing error {:#?}", e);
                Err(e)
            }
        }
    }
}

impl Stream for NatsTcpConn {
    type Item = ParserOp;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        match NatsTcpConn::decode(&mut this.read_buffer) {
            Ok(Some(op)) => return Poll::Ready(Some(op)),
            Ok(None) => {}
            Err(e) => {
                println!("could not decode: {}", e);
                return Poll::Ready(None);
            }
        }

        this.read_buffer.reserve(1);

        let mut buff: [u8; 2048] = [0; 2048];
        let mut buff: ReadBuf = ReadBuf::new(&mut buff);
        loop {
            match Pin::new(&mut this.stream).poll_read(cx, &mut buff) {
                Poll::Ready(Ok(())) => {
                    let filled = buff.filled();
                    let size = filled.len();
                    this.read_buffer.extend(filled);
                    buff.clear();
                    let read_buffer_contents =
                        std::str::from_utf8(this.read_buffer.as_ref()).unwrap();
                    println!("read_buffer: {}", read_buffer_contents);

                    if size > 0 {
                        if let Ok(Some(op)) = NatsTcpConn::decode(&mut this.read_buffer) {
                            return Poll::Ready(Some(op));
                        }
                    } else {
                        return Poll::Ready(None);
                    }
                }
                Poll::Ready(Err(err)) => {
                    return if err.kind() == io::ErrorKind::WouldBlock {
                        Poll::Pending
                    } else {
                        eprintln!("poll stream error");
                        Poll::Ready(None)
                    };
                }
                Poll::Pending => {
                    return Poll::Pending;
                }
            }
        }
    }
}

impl Sink<ParserOp> for NatsTcpConn {
    type Error = anyhow::Error;

    fn poll_ready(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::result::Result<(), Self::Error>> {
        if !self.flushed {
            match Pin::new(&mut self.get_mut().stream).poll_flush(cx)? {
                Poll::Ready(()) => Poll::Ready(Ok(())),
                Poll::Pending => Poll::Pending,
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

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::result::Result<(), Self::Error>> {
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

    fn poll_close(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::result::Result<(), Self::Error>> {
        ready!(self.poll_flush(cx))?;
        Poll::Ready(Ok(()))
    }
}
