//! Parsers for comments.
//! 
//! The entry point for this module is the [`comment`] function.
//! Single-line comments are denoted with the `#` character.

use nom::{
    combinator::map,
    character::complete::{char, line_ending, not_line_ending},
    sequence::delimited,
    multi::many0,
};

use crate::{
    ast::{Location, Located, Node, Literal},
    parser::{IResult, Span},
};

/// Parses a comment, returning a `Literal::Unit`.
pub fn comment(input: Span) -> IResult<Located<Node>> {
    map(
       delimited(char('#'), many0(not_line_ending), line_ending),
       |_| { Location::new( Literal::Unit, loc )} 
    )(input)
}