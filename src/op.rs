use std::borrow::Borrow;

use bytes::{BufMut, Bytes, BytesMut};
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

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct NatsInfoOp {
    #[serde(alias = "server_id")]
    pub(crate) server_id: String,
    #[serde(alias = "server_name")]
    pub(crate) server_name: Option<String>,
    pub(crate) version: String,
    pub(crate) go: String,
    pub(crate) host: String,
    pub(crate) port: usize,
    #[serde(alias = "max_payload")]
    pub(crate) max_payload: usize,
    pub(crate) proto: u8,
    #[serde(alias = "git_commit")]
    pub(crate) git_commit: Option<String>,
    #[serde(alias = "client_id")]
    pub(crate) client_id: Option<usize>,
    #[serde(alias = "auth_required")]
    pub(crate) auth_required: Option<bool>,
    #[serde(alias = "tls_required")]
    pub(crate) tls_required: Option<bool>,
    #[serde(alias = "tls_verified")]
    pub(crate) tls_verify: Option<bool>,
    #[serde(alias = "connect_urls")]
    pub(crate) connect_urls: Option<Vec<String>>,
    pub(crate) ldm: Option<bool>,

}

#[derive(Debug, PartialEq)]
pub enum ParserOp {
    Connect(NatsConnectOp),
    Info(NatsInfoOp),
    Ping,
    Pong,
    Ok,
}

impl ParserOp {
    pub fn into_bytes(self) -> anyhow::Result<Bytes> {
        println!("sending: {:#?}", self);
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
                let serialized_info = serde_json::to_string(info.borrow())?;

                let mut dst = BytesMut::with_capacity(prefix.len() + serialized_info.len() + 2);
                dst.put(prefix.as_bytes());
                dst.put(serialized_info.as_bytes());
                dst.put("\r\n".as_bytes());
                Ok(dst.freeze())
            }
            ParserOp::Ok => {
                println!("Sending OK");

                let prefix = "+OK";
                let mut dst = BytesMut::with_capacity(prefix.len() + 2);
                dst.put(prefix.as_bytes());
                dst.put("\r\n".as_bytes());
                Ok(dst.freeze())
            }
            ParserOp::Ping => {
                println!("Sending PING");

                let prefix = "PING";
                let mut dst = BytesMut::with_capacity(prefix.len() + 2);
                dst.put(prefix.as_bytes());
                dst.put("\r\n".as_bytes());
                Ok(dst.freeze())
            }
            ParserOp::Pong => {
                println!("Sending PONG");

                let prefix = "PONG";
                let mut dst = BytesMut::with_capacity(prefix.len() + 2);
                dst.put(prefix.as_bytes());
                dst.put("\r\n".as_bytes());
                Ok(dst.freeze())
            }
        }
    }
}