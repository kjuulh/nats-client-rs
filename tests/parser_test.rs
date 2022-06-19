use anyhow::{anyhow, Result};
use bytes::BytesMut;
use nom::{
    IResult,
    error::{context, ContextError, ErrorKind, FromExternalError, ParseError, VerboseError},
    Err::Error,
    combinator::{cut, into, map, map_res, opt, rest},
    character::is_space,
    character::complete::{alpha1, alphanumeric0, alphanumeric1, char, space0},
    bytes::complete::{tag, take_until, take_while},
    branch::alt,
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple}
};
use serde::{Deserialize, Serialize};
use nats_client_rs::parser;

#[test]
fn parse_connect() {
    let connect_raw = r#"[CONNECT {"verbose":false,"pedantic":false,"tls_required":false,"name":"","lang":"go","version":"1.2.2","protocol":1}]"#;

    let res = parser::Parser::parse(connect_raw);

    assert_eq!(true, res.is_ok())
}