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
    combinator::map,
    sequence::{delimited, pair, preceded, tuple},
};
use nom_locate::position;

use crate::{
    ast::{Block, Located, Location, Node},
    parser::{block::block1, IResult, Span},
};

/// Parses a [`Node::Cond`].
///
/// The spacing is explained in the other parsers of this module.
///
/// The location of the returned node matches the start of the `if` and the end of the `end`.
pub fn cond(input: Span) -> IResult<Located<Node>> {
    map(
        tuple((
            position,
            if_block,
            do_block,
            // FIXME: fix optional else block
            else_block,
            preceded(tag("end"), position),
        )),
        move |(sp1, if_block, do_block, else_block, sp2)| {
            Located::new(
                Node::Cond(if_block, do_block, else_block),
                Location::from(sp1) + Location::from(sp2),
            )
        },
    )(input)
}

/// Parses the `if` block of a [`Node::Cond`].
///
/// There must be at least one space or line break between the `if` and the first node in the
/// block. There can be spaces or line breaks at the end of the block.
///
/// The location of the returned block ignores the `if` and spaces surrounding the block.
fn if_block(input: Span) -> IResult<Located<Block>> {
    delimited(pair(tag("if"), multispace1), block1, multispace0)(input)
}

/// Parses the `do` block of a [`Node::Cond`].
///
/// There must be at least one space or line break between the `do` and the first node in the
/// block. There can be spaces or line breaks at the end of the block.
///
/// The location of the returned block ignores the `do` and spaces surrounding the block.
fn do_block(input: Span) -> IResult<Located<Block>> {
    delimited(pair(tag("do"), multispace1), block1, multispace0)(input)
}

/// Parses the `else` block of a [`Node::Cond`].
///
/// There must be at least one space or line break between the `else` and the first node in the
/// block. There can be spaces or line breaks at the end of the block.
///
/// The location of the returned block ignores the `else` and spaces surrounding the block.
fn else_block(input: Span) -> IResult<Located<Block>> {
    delimited(pair(tag("else"), multispace1), block1, multispace0)(input)
}
