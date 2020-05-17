//! Parsers for nodes.
//!
//! The entry-point for this module is the [`node`] parser. Each variant of the [`Node`] type has a
//! submodule here, with the exception of `Literal` and `Name` whose parsers are simply wrappers
//! over the [`literal`] and [`name`] parsers respectively.
//!
//! The [`binary_op`] module is particularly important here so it is a good idea to check those
//! module docs too.
//!
//! [`Node`]: crate::ast::Node
//! [`literal`]: crate::parser::literal
//! [`name`]: crate::parser::name
//! [`binary_op`]: module@crate::parser::node::binary_op
//! [`node`]: crate::parser::node::node

mod binary_op;
mod call;
mod cond;
mod fn_def;
mod fn_rec_def;
mod let_bind;
mod unary_op;

use nom::{error::ParseError, IResult};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space1,
    combinator::{cut, map, peek},
    sequence::{preceded, tuple},
};

use crate::ast::Node;

use crate::parser::{helpers::in_brackets, literal::literal, name::name, un_op::un_op};

use binary_op::binary_op;
use call::call;
use cond::cond;
use fn_def::fn_def;
use fn_rec_def::fn_rec_def;
use let_bind::let_bind;
use unary_op::unary_op;

/// Parser for [`Node`]s.
///
/// To understand its behaviour please refer to the [`module@binary_op`] docs.
pub fn node<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    binary_op(input)
}

/// Parser for base nodes and nodes inside brackets.
///
/// A base node is every node that is not a binary operation, i.e., all the variants of the
/// [`Node`] type are base nodes exceptuationg the `BinaryOp` variant.
///
/// This parser also does small lookaheads using the `peek` combinator to dispatch the parsers of
/// some variants and force those parsers to fail unrecoverably using the `cut` combinator. This
/// improves significantly the error messages generated by nom.
///
/// For nodes inside brackets, there can be any number of spaces between the brackets and the node.
fn base_node<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Node, E> {
    alt((
        (in_brackets(node)),
        map(literal, Node::Literal),
        preceded(peek(tag("if")), cut(cond)),
        preceded(
            peek(tuple((tag("fn"), space1, tag("rec")))),
            cut(fn_rec_def),
        ),
        preceded(peek(tag("fn")), cut(fn_def)),
        preceded(
            peek(name),
            cut(alt((let_bind, call, map(name, Node::Name)))),
        ),
        preceded(peek(un_op), cut(unary_op)),
    ))(input)
}