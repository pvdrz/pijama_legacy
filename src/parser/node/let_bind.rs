//! Parsers for let bindings.
//!
//! The entry point for this module is the [`let_bind`] function. Let bindings are parsed following
//! the rule
//!
//! ```abnf
//! let_bind = name (":" ty)? "=" node
//! ```
//!
//! Meaning that type bindings are optional.
use nom::{
    character::complete::{char, space0},
    combinator::{map, opt},
    sequence::{preceded, tuple},
};

use pijama_ast::{Located, Node, Span};

use crate::parser::{helpers::surrounded, name::name, node::node, ty::colon_ty, IResult};

/// Parses a [`Node::LetBind`].
///
/// There can be any number of spaces surrounding the `=` sign.
///
/// The location of the returned node matches the start of the name and the end of the node after
/// the `=`.
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
