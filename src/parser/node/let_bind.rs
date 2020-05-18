//! Parsers for let bindings.
//!
//! The entry-point for this module is the [`let_bind`] function. Conditionals are parsed with the
//! following grammar
//!
//! ```abnf
//! let_bind = name (":" ty)? "=" expr
//! ```
//!
//! Meaning that type bindings are optional.
//!
//! [`let_bind`]: crate::parser::node::let_bind::let_bind
//! [`Node::LetBind`]: crate::ast::Node::LetBind

use nom::{error::ParseError, IResult};

use nom::{
    character::complete::{char, space0},
    combinator::{map, opt},
    sequence::{preceded, tuple},
};

use crate::ast::Node;
use crate::parser::{helpers::surrounded, name::name, node::node, ty::colon_ty};

/// Parses a [`Node::LetBind`].
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
