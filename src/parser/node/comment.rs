//! Parsers for comments.
//! 
//! The entry point for this module is the [`comment`] function.
//! Single-line comments are denoted with the `#` character.

use nom::{
    combinator::value,
    character::complete::{char, line_ending, not_line_ending},
    sequence::delimited,
};

use crate::{
    parser::{IResult, Span},
};

/// Parses a comment, returning () if it finds one.
pub fn comment(input: Span) -> IResult<()> {
    value((), delimited(char('#'), not_line_ending, line_ending))(input)
}