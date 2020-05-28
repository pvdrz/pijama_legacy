//! Parsers for primitives.
//!
//! The entry point for this module is the [`primitive`] function. Primitives in Pijama must be
//! alphabetic `snake_case` strings.
//! They cannot be names and are listed in the [`PRIMITIVES`] constant.
use nom::{
    character::complete::{alpha1, char},
    combinator::{map, recognize, verify},
    multi::separated_nonempty_list,
};

use lazy_static::lazy_static;

use std::collections::HashMap;

use pijama_ast::{Located, Primitive};

use crate::{
    parser::{IResult, Span},
};

lazy_static! {
    /// Words that are primitives.
    pub static ref PRIMITIVES: HashMap<&'static str, Primitive> = {
        let mut m = HashMap::new();
        m.insert("print", Primitive::Print);
        m
    };
}

/// Parser for [`Primitive`]s.
///
/// The location of this element matches the start and end of its string slice in the source code.
pub fn primitive(input: Span) -> IResult<Located<Primitive>> {
    map(
        verify(
            recognize(separated_nonempty_list(char('_'), alpha1)),
            |span: &Span| PRIMITIVES.contains_key(span.fragment()),
        ),
        |span: Span| Located::new(*PRIMITIVES.get(span.fragment()).unwrap(), span),
    )(input)
}
