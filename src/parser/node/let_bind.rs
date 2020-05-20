//! Parsers for let bindings.
//!
//! The entry point for this module is the [`let_bind`] function. Let bindings are parsed following
//! the rule
//!
//! ```abnf
//! let_bind = name (":" ty)? "=" expr
//! ```
//!
//! Meaning that type bindings are optional.

use nom::{
    character::complete::{char, space0},
    combinator::{map, opt},
    error::ParseError,
    sequence::{preceded, tuple},
    IResult,
};

use crate::{
    ast::Node,
    parser::{helpers::surrounded, name::name, node::node, ty::colon_ty},
};

/// Parses a [`Node::LetBind`].
///
/// There can be any number of spaces surrounding the `=` sign.

pub fn let_bind<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    map(
        tuple((
            name,
            opt(colon_ty),
            preceded(surrounded(char('='), space0), node),
        )),
        |(name, opt_ty, node)| Node::LetBind(name, opt_ty, Box::new(node)),
    )(input)
}
