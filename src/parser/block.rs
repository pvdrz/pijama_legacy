//! Parsers for blocks.
//!
//! Blocks are sequences of nodes separated by at least one line break and optional spaces.

use nom::{error::ParseError, IResult};

use nom::{
    character::complete::{line_ending, multispace0},
    multi::{separated_list, separated_nonempty_list},
    sequence::preceded,
};

use crate::ast::Block;

use crate::parser::node::node;

/// Parser for [`Block`]s.
pub fn block0<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Block, E> {
    separated_list(line_ending, preceded(multispace0, node))(input)
}

/// Parser for non-empty [`Block`]s.
pub fn block1<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Block, E> {
    separated_nonempty_list(line_ending, preceded(multispace0, node))(input)
}
