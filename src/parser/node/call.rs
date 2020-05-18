//! Parsers for function calls.
//!
//! The entry point for this module is the [`call`] function. Function calls are parsed following
//! the rule
//!
//! ```abnf
//! call = "name "(" (node ("," node)*)? ")"
//! ```
use nom::{
    character::complete::space0, combinator::map, error::ParseError, sequence::separated_pair,
    IResult,
};
use nom_locate::position;

use crate::{
    ast::{Node, NodeKind, Span},
    parser::{
        name::name,
        node::{fn_def::args, node},
    },
};

/// Parses a [`Node::Call`].
///
/// This parser admits:
/// - Spaces after the name of the function.
/// - Spaces before and spaces or line breaks after each comma.
pub fn call<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Node, E> {
    let (input, span) = position(input)?;
    map(
        separated_pair(name, space0, args(node)),
        move |(name, args)| Node {
            kind: NodeKind::Call(name, args),
            span,
        },
    )(input)
}
