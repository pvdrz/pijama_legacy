//! Parsers for unary operations.
//!
//! The entry-point for this module is the [`unary_op`] parser. Currently there are no precedence
//! levels for these operations. It uses the [`un_op`] parser.
//!
//! [`un_op`]: crate::parser::un_op

use nom::{combinator::map, error::ParseError, sequence::pair, IResult};

use crate::{
    ast::Node,
    parser::{node::node, un_op::*},
};

/// Parses a [`Node::UnaryOp`].
pub fn unary_op<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    map(pair(un_op, node), |(un_op, node)| {
        Node::UnaryOp(un_op, Box::new(node))
    })(input)
}
