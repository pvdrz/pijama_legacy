//! Parsers for unary operations.
//!
//! The entry-point for this module is the [`unary_op`] parser. Currently there are no precedence
//! levels for these operations. It uses the [`un_op`] parser.
//!
//! [`unary_op`]: crate::parser::node::unary_op()
//! [`un_op`]: crate::parser::un_op
//! [`Node::UnaryOp`]: crate::ast::Node::UnaryOp

use nom::{error::ParseError, IResult};

use nom::{combinator::map, sequence::pair};

use crate::ast::Node;
use crate::parser::{node::node, un_op::*};

/// Parses a [`Node::UnaryOp`].
pub fn unary_op<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    map(pair(un_op, node), |(un_op, node)| {
        Node::UnaryOp(un_op, Box::new(node))
    })(input)
}
