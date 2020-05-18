//! Parsers for literals.
//!
//! The entry point for this module is the [`literal`] parser.

use nom::{error::ParseError, IResult};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_opt, opt},
    sequence::pair,
};

use crate::ast::Literal;

/// Parses a [`Literal`](crate::ast::Literal).
///
/// The only valid inputs for this parser are `"true"`, `"false"`, `"unit"` or a signed integer
/// (which is parsed by the [`number`](number) parser).
pub fn literal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Literal, E> {
    alt((
        map(tag("true"), |_| Literal::Bool(true)),
        map(tag("false"), |_| Literal::Bool(false)),
        map(tag("unit"), |_| Literal::Unit),
        map(number, Literal::Number),
    ))(input)
}

/// Parses a signed integer.
///
/// This integer must be in the valid range for the `i128` type. If the number is outside this
/// range, the parser will return an error.
///
/// If the number is negative, there cannot be spaces between the minus sign and the digits of the
/// number. That kind of expression will be parsed as an unary operation.
fn number<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, i128, E> {
    map_opt(
        pair(opt(char('-')), digit1),
        |(sign, digits): (Option<char>, &str)| {
            let mut number = digits.parse::<i128>().ok()?;
            if sign.is_some() {
                number *= -1;
            }
            Some(number)
        },
    )(input)
}
