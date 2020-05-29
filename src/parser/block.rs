//! Parsers for blocks.
//!
//! The [`block0`] parser is the top-level parser of the whole [`parser`] module.
//!
//! [`parser`]: crate::parser
use nom::{
    character::complete::{line_ending, multispace0},
    combinator::{map, opt},
    multi::{separated_list, separated_nonempty_list},
    sequence::{preceded, tuple},
};
use nom_locate::position;

use pijama_ast::{Block, Located, Location};

use crate::parser::{
    node::{comment, node},
    IResult, Span,
};

/// Parser for [`Block`]s which may or may not be empty.
///
/// Nodes in the block can be separated by at least one line break and optional spaces.
///
/// The location of this element matches either the start of a comment or the first space
/// or line break before the first `Node` of the `Block`. If there are no spaces or line
/// breaks before the first `Node`, the start matches the start of the `Node`. The end
/// of the location is handled in an analogous manner.
pub fn block0(input: Span) -> IResult<Located<Block>> {
    map(
        tuple((
            preceded(opt(comment::comment), position),
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
///
/// The location of this element matches either the start of a comment or the first space
/// or line break before the first `Node` of the `Block`. If there are no spaces or line
/// breaks before the first `Node`, the start matches the start of the `Node`. The end
/// of the location is handled in an analogous manner.
pub fn block1(input: Span) -> IResult<Located<Block>> {
    map(
        tuple((
            preceded(opt(comment::comment), position),
            separated_nonempty_list(line_ending, preceded(multispace0, node)),
            position,
        )),
        |(sp1, content, sp2)| {
            let loc = Location::from(sp1) + Location::from(sp2);
            Located::new(content, loc)
        },
    )(input)
}
