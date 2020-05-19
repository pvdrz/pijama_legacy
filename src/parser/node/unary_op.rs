//! Parsers for unary operations.
//!
//! The entry-point for this module is the [`unary_op`] parser. Currently there are no precedence
//! levels for these operations. It uses the [`un_op`] parser.
//!
//! [`un_op`]: crate::parser::un_op
use nom::{combinator::map, sequence::pair};
use nom_locate::position;

use crate::{
    ast::{Node, NodeKind, Span},
    parser::{node::node, un_op::*, IResult},
};

/// Parses a [`Node::UnaryOp`].
pub fn unary_op(input: Span) -> IResult<Node> {
    let (input, span) = position(input)?;
    map(pair(un_op, node), move |(un_op, node)| Node {
        kind: NodeKind::UnaryOp(un_op, Box::new(node)),
        span,
    })(input)
}
