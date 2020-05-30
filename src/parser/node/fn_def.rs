//! Parsers for function definitions.
//!
//! The entry point for this module is the [`fn_def`] function. Function definitions are parsed following the
//! rule
//!
//! ```abnf
//! fn_def = "fn" name? "(" (ty_annotation ("," ty_annotation)*)? ")" (":" ty)? "do" block1 "end"
//! ```
//!
//! Meaning that the return type annotation and name are optional. If the name is not given, the
//! expression will be interpreted as an anonymous function.
//!
//! The [`args`] parser is reutilized in the [`call`] parser.
//!
//! [`call`]: super::call
use nom::{
    character::complete::{char, multispace0, space0, space1},
    combinator::{map, opt},
    multi::separated_list,
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
};
use nom_locate::position;

use pijama_ast::{Block, Located, Location, Name, Node};

use crate::parser::{
    block::block0,
    helpers::{in_brackets, keyword, keyword_space, surrounded},
    name::name,
    ty::{colon_ty, ty_annotation},
    IResult, Span,
};

/// Parses a [`Node::FnDef`].
///
/// This parser admits:
/// - Spaces after the name of the function.
/// - Spaces or line breaks after the `")"` at the end of the arguments.
///
/// Other spacing details are in the docs for the other parsers of this module.
///
/// The location of the returned node matches the start of the `fn` and the end of the `end`.
pub fn fn_def(input: Span) -> IResult<Located<Node>> {
    map(
        tuple((
            fn_name,
            surrounded(args(ty_annotation), space0),
            terminated(opt(colon_ty), multispace0),
            fn_body,
        )),
        |(name, args, opt_ty, body)| {
            name.zip_with(body, move |name, body| {
                Node::FnDef(name, args.content, body, opt_ty)
            })
        },
    )(input)
}

/// Parses the name of a function in a definition if it has one.
///
/// This parser requires that the name is preceded by `"fn"` and at least one space. If the
/// function does not have a name, it need to parse the `"fn"` only.
///
/// The location of the returned node matches the start of the `fn` and the end of the name.
fn fn_name(input: Span) -> IResult<Located<Option<Located<Name>>>> {
    map(
        separated_pair(position, keyword("fn"), opt(preceded(space1, name))),
        |(span, opt_name)| {
            let mut loc = Location::from(span);
            if let Some(name) = opt_name.as_ref() {
                loc = loc + name.loc;
            }
            loc.with_content(opt_name)
        },
    )(input)
}

/// Parser for arguments of a function definition or function call.
///
/// - For function definitions: the arguments are type annotations.
/// - For function calls: the arguments are nodes.
///
/// These two options can be specified with the `content` parameter.
///
/// The arguments must be surrounded by brackets and seperated by commas. There can be spaces
/// before the comma and spaces or line breaks after the comma.
///
/// The location of the returned vector starts in `(` and ends in `)`.
pub fn args<'a, O: std::fmt::Debug>(
    content: impl Fn(Span<'a>) -> IResult<'a, O>,
) -> impl Fn(Span<'a>) -> IResult<'a, Located<Vec<O>>> {
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
///
/// The location of the returned vector starts in `do` and ends in `end`.
fn fn_body(input: Span) -> IResult<Located<Located<Block>>> {
    map(
        tuple((
            terminated(position, keyword_space("do")),
            block0,
            preceded(pair(multispace0, keyword("end")), position),
        )),
        |(sp1, content, sp2)| Located::new(content, Location::from(sp1) + Location::from(sp2)),
    )(input)
}
