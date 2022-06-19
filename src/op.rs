use anyhow::{anyhow, Result};
use bytes::BytesMut;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::{alpha1, alphanumeric0, alphanumeric1, char, space0},
    character::is_space,
    combinator::{cut, into, map, map_res, opt, rest},
    Err::Error,
    error::{context, ContextError, ErrorKind, FromExternalError, ParseError, VerboseError},
    IResult,
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
};
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
