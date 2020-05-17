//! Miscellaneous helper parsers.

use nom::{error::ParseError, IResult};

use nom::{
    character::complete::{char, multispace0},
    sequence::delimited,
};

/// Helper combinator to parse expressions surrounded by a delimiter.
///
/// The output only contains the expression without the delimiters.
pub fn surrounded<I, O, O2, E: ParseError<I>>(
    content: impl Fn(I) -> IResult<I, O, E>,
    delimiter: impl Fn(I) -> IResult<I, O2, E> + Copy,
) -> impl Fn(I) -> IResult<I, O, E> {
    delimited(delimiter, content, delimiter)
}

/// Helper combinator to parse expressions surrounded by round brackets.
///
/// There can be any number of spaces or line breaks between the actual content and the brackets. The output only
/// contains the expression without the backets.
pub fn in_brackets<'a, O, E: ParseError<&'a str>>(
    content: impl Fn(&'a str) -> IResult<&'a str, O, E>,
) -> impl Fn(&'a str) -> IResult<&'a str, O, E> {
    delimited(char('('), surrounded(content, multispace0), char(')'))
}
