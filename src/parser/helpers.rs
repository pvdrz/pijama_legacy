//! Miscellaneous helper parsers.

use nom::{
    character::complete::{char, multispace0},
    combinator::{cut, map, peek},
    error::ParseError,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
use nom_locate::position;

use crate::{
    ast::{Located, Location},
    parser::Span,
};

/// Helper parser for expressions surrounded by a delimiter.
///
/// The output only contains the expression without the delimiters.
pub fn surrounded<I, O, O2, E: ParseError<I>>(
    content: impl Fn(I) -> IResult<I, O, E>,
    delimiter: impl Fn(I) -> IResult<I, O2, E> + Copy,
) -> impl Fn(I) -> IResult<I, O, E> {
    delimited(delimiter, content, delimiter)
}

/// Helper parser for expressions surrounded by round brackets.
///
/// The output only contains the expression without the brackets and there can be any number of
/// spaces or line breaks between the actual content and the brackets.
pub fn in_brackets<'a, O: std::fmt::Debug, E: ParseError<Span<'a>>>(
    content: impl Fn(Span<'a>) -> IResult<Span<'a>, O, E>,
) -> impl Fn(Span<'a>) -> IResult<Span<'a>, Located<O>, E> {
    map(
        tuple((
            terminated(position, char('(')),
            surrounded(content, multispace0),
            preceded(char(')'), position),
        )),
        |(sp1, content, sp2)| {
            let loc = Location::new(sp1.location_offset(), sp2.location_offset() + 1);
            Located::new(content, loc)
        },
    )
}

/// Helper parser to do lookaheads.
///
/// This parser uses the [`peek`] combinator to check if the `hint` parser succeeds without
/// consuming the input. If that is the case, the
/// `content` parser is executed and it is forced to succeed using the [`cut`] combinator.
///
/// This is particularly useful when you are sure that there is only one expression that can be
/// parsed after a certain hint.
pub fn lookahead<'a, O, O2, E: ParseError<Span<'a>>>(
    hint: impl Fn(Span<'a>) -> IResult<Span<'a>, O2, E>,
    content: impl Fn(Span<'a>) -> IResult<Span<'a>, O, E>,
) -> impl Fn(Span<'a>) -> IResult<Span<'a>, O, E> {
    preceded(peek(hint), cut(content))
}
