//! Parsers for conditionals.
//!
//! The entry point for this module is the [`cond`] function. Conditionals are parsed following the
//! rule
//!
//! ```abnf
//! cond = "if" block1 "do" block1 ("else" block1)? "end"
//! ```
//!
//! Thus, `else` blocks are optional and are represented as empty [`Block`]s inside the
//! [`Node::Cond`] variant.
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    error::ParseError,
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};
use nom_locate::position;

use crate::{
    ast::{Block, Node, NodeKind, Span},
    parser::block::block1,
};

/// Parses a [`Node::Cond`].
///
/// The spacing is explained in the other parsers of this module.
pub fn cond<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Node, E> {
    let (input, span) = position(input)?;
    map(
        terminated(tuple((if_block, do_block, opt(else_block))), tag("end")),
        move |(if_block, do_block, else_block)| Node {
            span,
            kind: NodeKind::Cond(if_block, do_block, else_block.unwrap_or_default()),
        },
    )(input)
}

/// Parses the `if` block of a [`Node::Cond`].
///
/// There must be at least one space or line break between the `if` and the first node in the
/// block. There can be spaces or line breaks at the end of the block.
fn if_block<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Block, E> {
    delimited(pair(tag("if"), multispace1), block1, multispace0)(input)
}

/// Parses the `do` block of a [`Node::Cond`].
///
/// There must be at least one space or line break between the `do` and the first node in the
/// block. There can be spaces or line breaks at the end of the block.
fn do_block<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Block, E> {
    delimited(pair(tag("do"), multispace1), block1, multispace0)(input)
}

/// Parses the `else` block of a [`Node::Cond`].
///
/// There must be at least one space or line break between the `else` and the first node in the
/// block. There can be spaces or line breaks at the end of the block.
fn else_block<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Block, E> {
    delimited(pair(tag("else"), multispace1), block1, multispace0)(input)
}
