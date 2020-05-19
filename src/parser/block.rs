//! Parsers for blocks.
//!
//! The [`block0`] parser is the top-level parser of the whole [`parser`] module.
//!
//! [`parser`]: crate::parser
use nom::{
    character::complete::{line_ending, multispace0},
    multi::{separated_list, separated_nonempty_list},
    sequence::preceded,
};

use crate::{
    ast::{Block, Span},
    parser::{node::node, IResult},
};

/// Parser for [`Block`]s.
///
/// Nodes in the block can be separated by at least one line break and optional spaces.
pub fn block0(input: Span) -> IResult<Block> {
    separated_list(line_ending, preceded(multispace0, node))(input)
}

/// Parser for non-empty [`Block`]s.
///
/// Nodes in the block can be separated by at least one line break and optional spaces.
pub fn block1(input: Span) -> IResult<Block> {
    separated_nonempty_list(line_ending, preceded(multispace0, node))(input)
}
