//! Parsers for unary operations.
//!
//! The entry-point for this module is the [`unary_op`] parser. Currently there are no precedence
//! levels for these operations. It uses the [`un_op`] parser.
//!
//! [`un_op`]: crate::parser::un_op
use nom::{combinator::map, sequence::tuple};
use nom_locate::position;

use crate::{
    ast::{Located, Location, Node},
    parser::{node::node, un_op::un_op, IResult, Span},
};

/// Parses a [`Node::UnaryOp`].
pub fn unary_op(input: Span) -> IResult<Located<Node>> {
    map(tuple((position, un_op, node)), move |(sp, un_op, node)| {
        let loc = Location::from(sp) + node.loc;
        Located::new(Node::UnaryOp(un_op, Box::new(node)), loc)
    })(input)
}
