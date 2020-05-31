//! Parsers for literals.
//!
//! The entry point for this module is the [`literal`] parser.
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, hex_digit1},
    combinator::{map, map_opt, opt, value},
    sequence::tuple,
};
use nom_locate::position;

use pijama_ast::{Literal, Located, Location, Span};

use crate::parser::{helpers::with_context, IResult};

use std::borrow::Cow;

/// Parses a [`Literal`](crate::ast::Literal).
///
/// The only valid inputs for this parser are `"true"`, `"false"`, `"unit"` or a signed integer
/// (which is parsed by the [`number`](number) parser).
///
/// The location of this element matches the start and end of the inputs mentioned above inside the
/// source code.
pub fn literal(input: Span) -> IResult<Located<Literal>> {
    with_context(
        "Expected literal (true, false, unit) or number",
        alt((
            map(tag("true"), |span| Located::new(Literal::Bool(true), span)),
            map(tag("false"), |span| {
                Located::new(Literal::Bool(false), span)
            }),
            map(tag("unit"), |span| Located::new(Literal::Unit, span)),
            map(number, |located_num| located_num.map(Literal::Number)),
        )),
    )(input)
}

/// Determines the radix of the next number in the input.
///
/// Tries to parse a number prefix if possible otherwise it assumes 10 for decimal numbers.
fn number_radix(input: Span) -> IResult<u32> {
    alt((
        value(2, tag("0b")),
        value(8, tag("0o")),
        value(16, tag("0x")),
        |i| Ok((i, 10)),
    ))(input)
}

/// Parses a signed integer.
///
/// This integer must be in the valid range for the `i64` type. If the number is outside this
/// range, the parser will return an error.
///
/// If the number is negative, there cannot be spaces between the minus sign and the digits of the
/// number. That kind of expression will be parsed as an unary operation.
///
/// Numbers are accepted in decimal, binary, octal and hexadecimal notation.
/// Each of these besides decimal require a prefix. These are:
/// * binary `0b`
/// * octal `0o`
/// * hexadecimal `0x`
fn number(input: Span) -> IResult<Located<i64>> {
    map_opt(
        tuple((position, opt(char('-')), number_radix, hex_digit1)),
        |(position, sign, radix, digits_span): (Span, Option<char>, u32, Span)| {
            let number = if sign.is_some() {
                // Create a string with enough capacity for the number plus the sign to avoid unnecessary allocations when prepending the sign
                // This allows using the whole range of i64 numbers without handling the i64::min() case ourselves
                let mut number = String::with_capacity(digits_span.fragment().len() + 1);
                number.push('-');
                number.push_str(digits_span.fragment());
                Cow::from(number)
            } else {
                Cow::from(*digits_span.fragment())
            };
            let number = i64::from_str_radix(&number, radix).ok()?;
            let loc = Location::from(position) + digits_span.into();

            Some(loc.with_content(number))
        },
    )(input)
}
