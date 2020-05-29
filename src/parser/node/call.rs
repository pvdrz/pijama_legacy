//! Parsers for function calls.
//!
//! The entry point for this module is the [`call`] function. Function calls are parsed following
//! the rule
//!
//! ```abnf
//! call = "(name / "(" node ")") "(" (node ("," node)*)? ")"
//! ```
use nom::{branch::alt, character::complete::space0, combinator::map, sequence::separated_pair};

use pijama_ast::{Located, Node};

use crate::parser::{
    helpers::in_brackets,
    name::name,
    node::{fn_def::args, node},
    primitive::primitive,
    IResult, Span,
};

/// Parses a [`Node::Call`].
///
/// This parser admits:
/// - Spaces after the callee.
/// - Spaces before and spaces or line breaks after each comma.
///
/// The location of the returned node matches the start of the name and the end of the node after
/// the `=`.
pub fn call(input: Span) -> IResult<Located<Node>> {
    let func = alt((
        map(name, |Located { content, loc }| {
            Located::new(Node::Name(content), loc)
        }),
        map(primitive, |Located { content, loc }| {
            Located::new(Node::PrimFn(content), loc)
        }),
        map(in_brackets(node), |Located { mut content, loc }| {
            content.loc = loc;
            content
        }),
    ));
    map(separated_pair(func, space0, args(node)), |(func, args)| {
        let loc = func.loc + args.loc;
        Located::new(Node::Call(Box::new(func), args.content), loc)
    })(input)
}
