//! Parsers for unary operators.
//!
//! All the unary operators might be followed by zero or more spaces.
//!
//! The [`un_op`] parser is used inside the [`unary_op`] submodule.
//!
//! [`un_op`]: crate::parser::un_op::un_op
//! [`unary_op`]: module@crate::parser::node::unary_op
use nom::{error::ParseError, IResult};

use nom::{
    branch::alt,
    character::complete::{char, space0},
    combinator::map,
    sequence::terminated,
};

use crate::ast::UnOp::{self, *};

/// Parser for the unary operators `!` and `-`.
pub fn un_op<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, UnOp, E> {
    terminated(
        alt((map(char('!'), |_| Not), map(char('-'), |_| Neg))),
        space0,
    )(input)
}
