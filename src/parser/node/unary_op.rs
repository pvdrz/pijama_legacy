//! Parsers for unary operations.
//!
//! The entry-point for this module is the [`unary_op`] parser. Currently there are no precedence
//! levels for these operations. It uses the [`un_op`] parser.
//!
//! [`un_op`]: crate::parser::un_op
use nom::{combinator::map, sequence::tuple};
use nom_locate::position;

use pijama_ast::{Located, Location, Node};

use crate::parser::{node::node, un_op::un_op, IResult, Span};

/// Parses a [`Node::UnaryOp`].
///
/// The location of the returned node matches the start of the unary operation and the end of the inner node.
pub fn unary_op(input: Span) -> IResult<Located<Node>> {
    map(tuple((position, un_op, node)), move |(sp, un_op, node)| {
        let loc = Location::from(sp) + node.loc;
        Located::new(Node::UnaryOp(un_op, Box::new(node)), loc)
    })(input)
}
