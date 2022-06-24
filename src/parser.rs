use anyhow::{anyhow, Result};
use nom::{branch::alt, bytes::complete::tag, character::complete::{char, space0}, Err::Error, error::{context, ContextError, ErrorKind, FromExternalError, ParseError, VerboseError}, IResult, sequence::{delimited, pair}};
use nom::bytes::complete::take_till;
use serde::Deserialize;

use crate::op::{NatsConnectOp, NatsInfoOp, ParserOp};

pub struct Parser {}

impl Parser {
    pub fn parse<'a>(src: &'a str) -> anyhow::Result<ParserOp> {
        match Parser::root::<VerboseError<&'a str>>(src).map(|(_, op)| op) {
            Ok(op) => { Ok(op) }
            Err(e) => { Err(anyhow!("parsing error: {}", e)) }
        }
    }

    fn root<'a, E: ParseError<&'a str> + FromExternalError<&'a str, anyhow::Error> + ContextError<&'a str>>(i: &'a str) -> IResult<&'a str, ParserOp, E> {
        context(
            "container",
            alt(
                (
                    delimited(
                        char('['),
                        Parser::parse_container,
                        char(']'),
                    ),
                    Parser::parse_container,
                )
            ),
        )(i)
    }

fn parse_container<'a, E: ParseError<&'a str> + FromExternalError<&'a str, anyhow::Error> + ContextError<&'a str>>(i: &'a str) -> IResult<&'a str, ParserOp, E> {
    let (input, (command, _)) = pair(
        alt(
            (
                tag("INFO"),
                tag("CONNECT"),
                tag("MSG"),
                tag("PING"),
                tag("PONG"),
                tag("+OK"),
                tag("-ERR")
            )
        ),
        space0)(i)?;

    Parser::parse_command(command, input)
}

fn parse_command<'a, E: ParseError<&'a str> + FromExternalError<&'a str, anyhow::Error> + ContextError<&'a str>>(command: &'a str, i: &'a str) -> IResult<&'a str, ParserOp, E> {
    println!("Receiving: {}", command);
    let res = match command {
        "CONNECT" => Parser::parse_json::<NatsConnectOp>(i).map(ParserOp::Connect),
        "INFO" => Parser::parse_json::<NatsInfoOp>(i).map(ParserOp::Info),
        "PING" => Ok(ParserOp::Ping),
        "PONG" => Ok(ParserOp::Pong),
        "+OK" => Ok(ParserOp::Ok),
        _ => Err(anyhow!("COMMAND not allowed in nats protocol"))
    };

    match res {
        Ok(res) => Ok(("", res)),
        Err(e) => Err(Error(FromExternalError::from_external_error(i, ErrorKind::TakeUntil, e))),
    }
}

fn parse_json<'a, A>(s: &'a str) -> Result<A> where A: Deserialize<'a> {
    let parsing_string = s.trim_end_matches(']');
    Ok(serde_json::from_str::<A>(parsing_string)?)
}
}
