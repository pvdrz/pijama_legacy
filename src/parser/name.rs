//! Parsers for names.
//!
//! The entry-point for this module is the [`name`] function. Names of variables in Pijama must be
//! alphabetic `snake-case` strings. Certain keywords such as `fn`, `do` and `end` cannot be names,
//! such keywords are in the [`KEYWORDS`] constant.
//!
//! [`name`]: crate::parser::name::name.
//! [`KEYWORDS`]: crate::parser::name::KEYWORDS.
//! [`Name`]: crate::ast::Name
use nom::{error::ParseError, IResult};

use nom::{
    character::complete::{alpha1, char},
    combinator::{map, recognize, verify},
    multi::separated_nonempty_list
};

use crate::ast::Name;

/// Words that cannot be names to avoid ambiguities.
const KEYWORDS: &[&str] = &[
    "fn", "do", "end", "if", "else", "true", "false", "unit", "Bool", "Int", "Unit", "rec",
];

/// Parser for [`Name`]s.
pub fn name<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Name<'a>, E> {
    map(
        verify(recognize(separated_nonempty_list(char('_'), alpha1)), |s| {
            !KEYWORDS.contains(s)
        }),
        Name,
    )(input)
}
