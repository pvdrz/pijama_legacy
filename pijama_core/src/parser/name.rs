//! Parsers for names.
//!
//! The entry point for this module is the [`name`] function. Names of variables in Pijama must be
//! alphabetic `snake_case` strings. Certain keywords such as `fn`, `do` and `end` and primitive
//! functions cannot be names, these are listed in the [`KEYWORDS`] or [`PRIMITIVES`] constants.
use nom::{
    character::complete::{alpha1, char},
    combinator::{map, recognize, verify},
    multi::separated_nonempty_list,
};

use pijama_ast::{Located, Name, Span};

use crate::parser::{primitive::PRIMITIVES, IResult};

/// Words that cannot be names to avoid ambiguities.
const KEYWORDS: &[&str] = &[
    "fn", "rec", "do", "end", "if", "elif", "else", "true", "false", "unit", "Bool", "Int", "Unit",
];

/// Parser for [`Name`]s.
///
/// This parser is the main reason why most of the types and functions in the language are generic
/// over the `'a` lifetime. It allows to do zero-copy parsing and keep using the string slices
/// for the names through all the compilation process.
///
/// The location of this element matches the start and end of its string slice in the source code.
pub fn name(input: Span) -> IResult<Located<Name>> {
    verify(
        map(
            recognize(separated_nonempty_list(char('_'), alpha1)),
            |span: Span| Located::new(Name(span.fragment()), span),
        ),
        |name| !KEYWORDS.contains(&name.content.0) && !PRIMITIVES.contains_key(&name.content.0),
    )(input)
}
