//! Parsers for unary operators.
//!
//! The [`un_op`] parser is used inside the [`unary_op`] submodule.
//!
//! [`un_op`]: crate::parser::un_op::un_op
//! [`unary_op`]: crate::parser::node::unary_op
use nom::{
    branch::alt,
    character::complete::{char, space0},
    combinator::map,
    sequence::terminated,
};

use pijama_ast::{Span, UnOp, UnOp::*};

use crate::parser::{helpers::with_context, IResult};

/// Parser for the unary operators `!` and `-`.
///
/// All the unary operators might be followed by zero or more spaces.
pub fn un_op(input: Span) -> IResult<UnOp> {
    terminated(
        with_context(
            "Expected unary operator (!, -)",
            alt((map(char('!'), |_| Not), map(char('-'), |_| Neg))),
        ),
        space0,
    )(input)
}
