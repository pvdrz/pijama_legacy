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
    sequence::{preceded, tuple},
};

use crate::{
    ast::{Located, Node},
    parser::{helpers::surrounded, name::name, node::node, ty::colon_ty, IResult, Span},
};

/// Parses a [`Node::LetBind`].
///
/// There can be any number of spaces surrounding the `=` sign.
pub fn let_bind(input: Span) -> IResult<Located<Node>> {
    map(
        tuple((
            name,
            opt(colon_ty),
            preceded(surrounded(char('='), space0), node),
        )),
        |(name, opt_ty, node)| {
            let loc = name.loc + node.loc;
            Located::new(Node::LetBind(name, opt_ty, Box::new(node)), loc)
        },
    )(input)
}
