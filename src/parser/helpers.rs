//! Miscellaneous helper parsers.

use nom::{
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1},
    combinator::{cut, map, peek},
    error::ParseError,
    sequence::{delimited, pair, preceded, terminated, tuple},
    Compare, IResult, InputLength,
};
use nom_locate::position;

use crate::{
    ast::{Located, Location},
    parser::{ParsingError, Span},
};

use std::fmt::Display;

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
///
/// The location of this element starts in the `(` and ends in the `)`.
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

/// Helper parser for a keyword.
///
/// This parses using the ['tag'] combinator to check for an expected keyword
/// and provides additional context in case it is missing.
pub fn keyword<'a, T: 'a>(
    t: T,
) -> impl Fn(Span<'a>) -> IResult<Span<'a>, Span<'a>, ParsingError<'a>>
where
    T: InputLength + Clone + Display,
    Span<'a>: Compare<T>,
{
    with_context(format!("Expected keyword {}.", t), tag(t))
}

/// Helper parser for a keyword with at least one following whitespace.
///
/// This parses using the ['pair'] combinator to check for an expected keyword and following spaces
/// and provides additional context in case the keyword or the whitespace is missing.
pub fn keyword_space<'a, T: 'a>(
    t: T,
) -> impl Fn(Span<'a>) -> IResult<Span<'a>, (Span<'a>, Span<'a>), ParsingError<'a>>
where
    T: InputLength + Clone + Display,
    Span<'a>: Compare<T>,
{
    pair(
        keyword(t.clone()),
        with_context(format!("Space required after keyword {}.", t), multispace1),
    )
}

/// Helper function to associate a context to the `ParsingError` when `inner` fails.
pub fn with_context<'a, O>(
    context: impl ToString,
    inner: impl Fn(Span<'a>) -> IResult<Span<'a>, O, ParsingError<'a>>,
) -> impl Fn(Span<'a>) -> IResult<Span<'a>, O, ParsingError<'a>> {
    move |i| match inner(i) {
        Ok(o) => Ok(o),
        Err(e) => Err(e.map(|error| ParsingError::with_context(i, context.to_string(), error))),
    }
}
