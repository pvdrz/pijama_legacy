//! Parsers for function definitions.
//!
//! The entry point for this module is the [`fn_def`] function. Function definitions and anonymous
//! functions are parsed following the rules
//!
//! ```abnf
//! fn_def = "fn" name "(" (ty_annotation ("," ty_annotation)*)? ")" (":" ty)? "do" block1 "end"
//! anon_fn = "fn" "(" (ty_annotation ("," ty_annotation)*)? ")" (":" ty)? "do" block1 "end"
//! ```
//!
//! The `fn_def` parser takes care of both rules: If the name is not given, the expression will be
//! interpreted as an anonymous function.
//!
//! The [`args`] parser is reutilized in the [`call`] parser.
//!
//! [`call`]: super::call
use nom::{
    character::complete::{char, multispace0, space0, space1},
    combinator::{map, opt},
    multi::separated_list,
    sequence::{delimited, pair, preceded, terminated, tuple},
};
use nom_locate::position;

use pijama_ast::{ty::TyAnnotation, Block, Located, Location, Node, Span};

use crate::parser::{
    block::block0,
    helpers::{in_brackets, keyword, keyword_space, surrounded},
    name::name,
    ty::{colon_ty, ty_annotation},
    IResult,
};

/// Parses a [`Node::FnDef`] or [`Node::AnonFn`].
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
            keyword("fn"),
            opt(preceded(space1, name)),
            surrounded(args(ty_annotation), space0),
            terminated(colon_ty, multispace0),
            fn_body,
        )),
        |(fn_kw, opt_name, args, ty, body)| {
            Location::from(fn_kw)
                .with_content(())
                .zip_with(body, move |_, body| {
                    if let Some(name) = opt_name {
                        Node::FnDef(name, args.content, TyAnnotation { item: body, ty })
                    } else {
                        Node::AnonFn(args.content, TyAnnotation { item: body, ty })
                    }
                })
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
