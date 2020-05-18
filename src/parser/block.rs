//! Parsers for blocks.
//!
//! The [`block0`] parser is the top-level parser of the whole [`parser`] module.
//!
//! [`parser`]: crate::parser

use nom::{error::ParseError, IResult};

use nom::{
    character::complete::{line_ending, multispace0},
    multi::{separated_list, separated_nonempty_list},
    sequence::preceded,
};

use crate::ast::Block;

use crate::parser::node::node;

/// Parser for [`Block`]s.
///
/// Nodes in the block can be separated by at least one line break and optional spaces.
pub fn block0<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Block, E> {
    separated_list(line_ending, preceded(multispace0, node))(input)
}

/// Parser for non-empty [`Block`]s.
///
/// Nodes in the block can be separated by at least one line break and optional spaces.
pub fn block1<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Block, E> {
    separated_nonempty_list(line_ending, preceded(multispace0, node))(input)
}
