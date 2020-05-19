//! Parsers for unary operations.
//!
//! The entry-point for this module is the [`unary_op`] parser. Currently there are no precedence
//! levels for these operations. It uses the [`un_op`] parser.
//!
//! [`un_op`]: crate::parser::un_op
use nom::{combinator::map, sequence::pair};
use nom_locate::position;

use crate::{
    ast::{Location, Node, NodeKind},
    parser::{node::node, un_op::un_op, IResult, Span},
};

/// Parses a [`Node::UnaryOp`].
pub fn unary_op(input: Span) -> IResult<Node> {
    let (_, span) = position(input)?;
    let start = span.location_offset();

    map(pair(un_op, node), move |(un_op, node)| {
        let end = node.loc.end;
        Node::new(
            NodeKind::UnaryOp(un_op, Box::new(node)),
            Location::new(start, end),
        )
    })(input)
}
