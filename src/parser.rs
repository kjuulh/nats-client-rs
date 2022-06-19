use anyhow::{anyhow, Result};
use nom::{branch::alt, bytes::complete::tag, character::complete::{char, space0}, Err::Error, error::{context, ContextError, ErrorKind, FromExternalError, ParseError, VerboseError}, IResult, sequence::{delimited, pair}};
use serde::Deserialize;

use crate::op::{NatsConnectOp, ParserOp};

pub struct Parser {}

impl Parser {
    pub fn parse<'a>(src: &'a str) -> anyhow::Result<ParserOp> {
        Parser::root::<VerboseError<&'a str>>(src).map(|(_, op)| op)
            .or(Err(anyhow!("parsing error")))
    }

    fn root<'a, E: ParseError<&'a str> + FromExternalError<&'a str, anyhow::Error> + ContextError<&'a str>>(i: &'a str) -> IResult<&'a str, ParserOp, E> {
        context("container", delimited(
            char('['),
            Parser::parse_container,
            char(']'),
        ))(i)
    }

    fn parse_container<'a, E: ParseError<&'a str> + FromExternalError<&'a str, anyhow::Error> + ContextError<&'a str>>(i: &'a str) -> IResult<&'a str, ParserOp, E> {
        let (input, (command, _)) = pair(
            alt(
                (
                    tag("CONNECT"),
                    tag("INFO"),
                )
            ),
            space0)(i)?;

        Parser::parse_command(command, input)
    }

    fn parse_command<'a, E: ParseError<&'a str> + FromExternalError<&'a str, anyhow::Error> + ContextError<&'a str>>(command: &'a str, i: &'a str) -> IResult<&'a str, ParserOp, E> {
        let res = match command {
            "CONNECT" => Parser::parse_json::<NatsConnectOp>(i).map(ParserOp::Connect),
            _ => Err(anyhow!("COMMAND not allowed in nats protocol"))
        };

        match res {
            Ok(res) => Ok(("]", res)),
            Err(e) => Err(Error(FromExternalError::from_external_error(i, ErrorKind::TakeUntil, e))),
        }
    }

    fn parse_json<'a, A>(s: &'a str) -> Result<A> where A: Deserialize<'a> {
        let parsing_string = s.trim_end_matches(']');
        Ok(serde_json::from_str::<A>(parsing_string)?)
    }
}
