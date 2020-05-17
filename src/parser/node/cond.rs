//! Parsers for conditionals.
//!
//! The entry-point for this module is the [`cond`] function. Conditionals are parsed with the
//! following grammar
//!
//! ```abnf
//! cond = "if" block1 "do" block1 ("else" block1)? "end"
//! ```
//!
//! Thus, `else` blocks are optional and are represented as empty [`Block`]s inside the
//! [`Node::Cond`] variant.
//!
//! [`cond`]: crate::parser::node::cond::cond
//! [`Block`]: crate::ast::Block
//! [`Node::Cond`]: crate::ast::Node::Cond
use nom::{error::ParseError, IResult};

use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    sequence::{delimited, pair, terminated, tuple},
};

use crate::ast::{Block, Node};
use crate::parser::block::block1;

/// Parses a [`Node::Cond`].
pub fn cond<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    map(
        terminated(tuple((if_block, do_block, opt(else_block))), tag("end")),
        |(if_block, do_block, else_block)| {
            Node::Cond(if_block, do_block, else_block.unwrap_or_default())
        },
    )(input)
}

/// Parses the `if` block of a [`Node::Cond`].
///
/// There must be at least one space or line break between the `if` and the first node in the
/// block. There can be spaces or line breaks at the end of the block.
fn if_block<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Block, E> {
    delimited(pair(tag("if"), multispace1), block1, multispace0)(input)
}

/// Parses the `do` block of a [`Node::Cond`].
///
/// There must be at least one space or line break between the `if` and the first node in the
/// block. There can be spaces or line breaks at the end of the block.
fn do_block<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Block, E> {
    delimited(pair(tag("do"), multispace1), block1, multispace0)(input)
}

/// Parses the `else` block of a [`Node::Cond`].
///
/// There must be at least one space or line break between the `if` and the first node in the
/// block. There can be spaces or line breaks at the end of the block.
fn else_block<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Block, E> {
    delimited(pair(tag("else"), multispace1), block1, multispace0)(input)
}
