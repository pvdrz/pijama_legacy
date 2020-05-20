//! Parsers for names.
//!
//! The entry point for this module is the [`name`] function. Names of variables in Pijama must be
//! alphabetic `snake-case` strings. Certain keywords such as `fn`, `do` and `end` cannot be names,
//! such keywords are in the [`KEYWORDS`] constant.
use nom::{
    character::complete::{alpha1, char},
    combinator::{map, recognize, verify},
    error::ParseError,
    multi::separated_nonempty_list,
    IResult,
};

use crate::ast::Name;

/// Words that cannot be names to avoid ambiguities.
const KEYWORDS: &[&str] = &[
    "fn", "rec", "do", "end", "if", "else", "true", "false", "unit", "Bool", "Int", "Unit",
];

/// Parser for [`Name`]s.
///
/// This parser is the main reason why most of the types and functions in the language are generic
/// over the `'a` lifetime. It allows to do zero-copy parsing and keep using the string slices
/// for the names through all the compilation process.
pub fn name<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Name<'a>, E> {
    map(
        verify(recognize(separated_nonempty_list(char('_'), alpha1)), |s| {
            !KEYWORDS.contains(s)
        }),
        Name,
    )(input)
}
