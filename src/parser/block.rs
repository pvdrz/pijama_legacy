//! Parsers for blocks.
//!
//! The [`block0`] parser is the top-level parser of the whole [`parser`] module.
//!
//! [`parser`]: crate::parser
use nom::{
    character::complete::{line_ending, multispace0},
    combinator::map,
    multi::{separated_list, separated_nonempty_list},
    sequence::{preceded, tuple},
};
use nom_locate::position;

use crate::{
    ast::{Block, Located, Location},
    parser::{node::node, IResult, Span},
};

/// Parser for [`Block`]s.
///
/// Nodes in the block can be separated by at least one line break and optional spaces.
pub fn block0(input: Span) -> IResult<Located<Block>> {
    map(
        tuple((
            position,
            separated_list(line_ending, preceded(multispace0, node)),
            position,
        )),
        |(sp1, content, sp2)| {
            let loc = Location::from(sp1) + Location::from(sp2);
            Located::new(content, loc)
        },
    )(input)
}

/// Parser for non-empty [`Block`]s.
///
/// Nodes in the block can be separated by at least one line break and optional spaces.
pub fn block1(input: Span) -> IResult<Located<Block>> {
    map(
        tuple((
            position,
            separated_nonempty_list(line_ending, preceded(multispace0, node)),
            position,
        )),
        |(sp1, content, sp2)| {
            let loc = Location::from(sp1) + Location::from(sp2);
            Located::new(content, loc)
        },
    )(input)
}
