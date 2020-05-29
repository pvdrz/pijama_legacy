//! Parsers for conditionals.
//!
//! The entry point for this module is the [`cond`] function. Conditionals are parsed following the
//! rule
//!
//! ```abnf
//! cond = "if" block1 "do" block1 ("elif" block1 "do" block1)* "else" block1 "end"
//! ```
//!
//! Thus, `elif` blocks are optional and are represented as empty [`Block`]s inside the
//! [`Node::Cond`] variant.
use nom::{
    character::complete::multispace0,
    combinator::map,
    multi::many0,
    sequence::{delimited, pair, preceded, tuple},
};
use nom_locate::position;

use pijama_ast::{Block, Branch, Located, Location, Node};

use crate::parser::{
    block::block1,
    helpers::{keyword, keyword_space},
    IResult, Span,
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
            branch("if"),
            many0(branch("elif")),
            keyword_block("else"),
            preceded(keyword("end"), position),
        )),
        move |(sp1, if_branch, branches, else_block, sp2)| {
            Located::new(
                Node::Cond(if_branch, branches, else_block),
                Location::from(sp1) + Location::from(sp2),
            )
        },
    )(input)
}

/// Parses a `Branch` representing either an "if do" conditional statement or an "elif do" statement.
fn branch<'a>(keyword: &'a str) -> impl Fn(Span<'a>) -> IResult<Branch<'a>> {
    map(
        pair(keyword_block(keyword), keyword_block("do")),
        |(blk1, blk2)| Branch {
            cond: blk1,
            body: blk2,
        },
    )
}

/// Parses the `keyword` block of a [`Node::Cond`].
///
/// There must be at least one space or line break between the `keyword` and the first node in the
/// block. There can be spaces or line breaks at the end of the block.
///
/// The location of the returned block ignores the `keyword` and spaces surrounding the block.
fn keyword_block<'a>(keyword: &'a str) -> impl Fn(Span<'a>) -> IResult<Located<Block<'a>>> {
    delimited(keyword_space(keyword), block1, multispace0)
}
