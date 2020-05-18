//! Parsers for function definitions.
//!
//! The entry point for this module is the [`fn_def`] function. Function definitions are parsed following the
//! rule
//!
//! ```abnf
//! fn_def = "fn" name? "(" (binding ("," binding)*)? ")" (":" ty)? "do" block1 "end"
//! ```
//!
//! Meaning that the return type binding and name are optional. If the name is not given, the
//! expression will be interpreted as an anonymous function.
//!
//! The [`fn_body`] and [`args`] parsers are reutilized in the [`fn_rec_def`] and [`call`] parsers.
//!
//! [`fn_rec_def`]: super::fn_rec_def
//! [`call`]: super::call
use nom::{
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1, space0, space1},
    combinator::{map, opt},
    error::ParseError,
    multi::separated_list,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
use nom_locate::position;

use crate::{
    ast::{Block, Name, Node, NodeKind, Span},
    parser::{
        block::block0,
        helpers::{in_brackets, surrounded},
        name::name,
        ty::{binding, colon_ty},
    },
};

/// Parses a [`Node::FnDef`].
///
/// This parser admits:
/// - Spaces after the name of the function.
/// - Spaces or line breaks after the `")"` at the end of the arguments.
///
/// Other spacing details are in the docs for the other parsers of this module.
pub fn fn_def<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Node, E> {
    let (input, span) = position(input)?;
    map(
        tuple((
            fn_name,
            surrounded(args(binding), space0),
            terminated(opt(colon_ty), multispace0),
            fn_body,
        )),
        move |(name, args, opt_ty, body)| Node {
            kind: NodeKind::FnDef(name, args, body, opt_ty),
            span,
        },
    )(input)
}

/// Parses the name of a function in a definition if it has one.
///
/// This parser requires that the name is preceded by `"fn"` and at least one space. If the
/// function does not have a name, it need to parse the `"fn"` only.
fn fn_name<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Option<Name>, E> {
    preceded(tag("fn"), opt(preceded(space1, name)))(input)
}

/// Parser for arguments of a function definition or function call.
///
/// - For function definitions: the arguments are bindings.
/// - For function calls: the arguments are nodes.
///
/// These two options can be specified with the `content` parameter.
///
/// The arguments must be surrounded by brackets and seperated by commas. There can be spaces
/// before the comma and spaces or line breaks after the comma.
pub fn args<'a, O, E: ParseError<Span<'a>>>(
    content: impl Fn(Span<'a>) -> IResult<Span<'a>, O, E>,
) -> impl Fn(Span<'a>) -> IResult<Span<'a>, Vec<O>, E> {
    in_brackets(separated_list(
        delimited(space0, char(','), multispace0),
        content,
    ))
}

/// Parses the body of a function definition.
///
/// The body is parsed as a `Block`. This parser requires that the body is preceded by `"do"` and
/// at least one space or line break, and followed by zero or more spaces or line breaks and an
/// `"end"`.
pub fn fn_body<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Block, E> {
    delimited(
        pair(tag("do"), multispace1),
        block0,
        pair(multispace0, tag("end")),
    )(input)
}
