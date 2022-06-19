use std::borrow::Borrow;
use bytes::{BufMut, Bytes, BytesMut};
use nom::AsBytes;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct NatsConnectOp {
    pub(crate) verbose: bool,
    pub(crate) pedantic: bool,
    #[serde(alias = "tls_required")]
    pub(crate) tls_required: bool,
    pub(crate) name: String,
    pub(crate) lang: String,
    pub(crate) version: String,
    pub(crate) protocol: u8,
}

#[derive(Debug, PartialEq)]
pub enum ParserOp {
    Connect(NatsConnectOp),
    Info(String),
}

#[derive(thiserror::Error, Debug)]
pub enum NatsParsingError {
    #[error("COMMAND not allowed in NATS client")]
    CommandNotAllowed
}


impl ParserOp {
    pub fn into_bytes(self) -> anyhow::Result<Bytes> {
        match self {
            ParserOp::Connect(conn) => {
                let prefix = "CONNECT";
                let serialized_connect = serde_json::to_string(conn.borrow())?;

                let mut dst = BytesMut::with_capacity(prefix.len() + serialized_connect.len() + 2);
                dst.put(prefix.as_bytes());
                dst.put(serialized_connect.as_bytes());
                dst.put("\r\n".as_bytes());
                Ok(dst.freeze())
            }
            ParserOp::Info(info) => {
                let prefix = "INFO";
                let serialized_connect = info;

                let mut dst = BytesMut::with_capacity(prefix.len() + serialized_connect.len() + 2);
                dst.put(prefix.as_bytes());
                dst.put(serialized_connect.as_bytes());
                dst.put("\r\n".as_bytes());
                Ok(dst.freeze())
            }
        }
    }
}