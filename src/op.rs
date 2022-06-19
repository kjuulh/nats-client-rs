use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct NatsConnectOp {
    verbose: bool,
    pedantic: bool,
    #[serde(alias = "tls_required")]
    tls_required: bool,
    name: String,
    lang: String,
    version: String,
    protocol: u8,
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
