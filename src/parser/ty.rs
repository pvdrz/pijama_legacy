//! Parsers for types and type bindings.
//!
//! The entry points for this module are the [`ty`] and [`binding`] parsers. We need an additional
//! [`base_ty`] parser because the naive grammar for types is a
//! [left-recursive](https://en.wikipedia.org/wiki/Left_recursion) grammar:
//!
//! ```abnf
//! ty = (ty "->" ty) / "Bool" / "Int" / "Unit" / ("(" ty ")")
//! ```
//!
//! This means that, with the above grammar, the first thing that the parser tries when to do when
//! parsing a type, is again parsing a type, which ends up causing an infinite loop. For this
//! reason the grammar needs to be rewritten to not have left-recursion:
//!
//! ```abnf
//! ty = base_ty ("->" ty)*
//! base_ty = "Bool" / "Int" / "Unit" / ("(" ty ")")
//! ```
//!
//! Now the first thing done when parsing a type is trying to parse a "base type" avoiding the loop
//! completely. The [`ty`] and [`base_ty`] parsers in this module corresponds to each one of the
//! rules in the grammar above.
//!
//! In addition we have the [`ty_annotation`] parser, which parses expressions with the grammar
//!
//! ```abnf
//! ty_annotation = name ":" ty
//! ```
//! The parser for names is explained in the [`name`] module.
//!
//! [`ty`]: crate::parser::ty::ty
//! [`base_ty`]: crate::parser::ty::base_ty
//! [`ty_annotation`]: crate::parser::ty::ty_annotation
//! [`Ty`]: crate::ty::Ty
//! [`TyAnnotation`]: crate::ty::TyAnnotation
//! [`name`]: crate::parser::name
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, space0, space1},
    combinator::{cut, map, opt},
    sequence::{pair, preceded, separated_pair, terminated},
};

use nom_locate::position;

use pijama_ast::{
    ty::{Ty, TyAnnotation},
    Located, Location, Name, Span,
};

use crate::parser::{
    helpers::{in_brackets, surrounded, with_context},
    name::name,
    IResult,
};

/// Parser for all types.
///
/// This parser has almost the same behavior as [`base_ty`] but it also allows function types.
///
/// The `->` operator is right-associative, which means that `Bool -> Int -> Unit` is equal to
/// `Bool -> (Int -> Unit)`.
///
/// There can be any number of spaces surrounding the `->`, including no spaces at all.
///
/// If the returned type is an `Arrow`, the location matches the start of the first type
/// and the end of the second. For any other case refer to the [`base_ty`] docs.
pub fn ty(input: Span) -> IResult<Located<Ty>> {
    let (rem, t1) = base_ty(input)?;
    if let (rem, Some(t2)) = opt(preceded(surrounded(tag("->"), space0), ty))(rem)? {
        Ok((
            rem,
            t1.zip_with(t2, |t1, t2| Ty::Arrow(Box::new(t1), Box::new(t2))),
        ))
    } else {
        Ok((rem, t1))
    }
}

/// Parser for name type annotations.
///
/// This parser returns a [`TyAnnotation<Name>`], there can be any number of spaces surrounding the `:`,
/// including no spaces at all.
pub fn ty_annotation(input: Span) -> IResult<TyAnnotation<Name<'_>>> {
    map(
        separated_pair(name, surrounded(char(':'), space0), ty),
        |(name, ty)| TyAnnotation { item: name, ty },
    )(input)
}

/// Parses a type preceded by a colon if it exists.
///
/// This parser returns a [`Ty`] and there can be any number of spaces surrounding the colon. If
/// there is no colon and [`Ty`] it returns `Ty::Missing` located where the parsing started.
///
/// This parser exists with the sole purpose of being reutilized for type annotations that are not
/// stored in [`TyAnnotation`]s such as the return type of functions or the optional type
/// annotation for let bindings.
pub fn colon_ty(input: Span) -> IResult<Located<Ty>> {
    map(
        pair(
            position,
            opt(preceded(
                surrounded(char(':'), space0),
                cut(terminated(ty, space1)),
            )),
        ),
        |(span, opt_ty)| opt_ty.unwrap_or_else(|| Location::from(span).with_content(Ty::Missing)),
    )(input)
}

/// Parser for base types and types in brackets.
///
/// The only valid inputs for this parser are `"Bool"`, `"Int"`, `"Unit"` and a type surrounded by
/// round brackets. It returns a [`Ty`].
///
/// There can be any number of spaces between the brackets and its contents.
///
/// If the returned type is one of the string slices mentioned above, the location matches the one
/// of the slice. If the returned type is surrounded by brackets, the location matches the span of
/// the brackets.
fn base_ty(input: Span) -> IResult<Located<Ty>> {
    with_context(
        "Expected basic type (Bool, Int, Unit) or type in brackets",
        alt((
            map(tag("Bool"), |span: Span| Located::new(Ty::Bool, span)),
            map(tag("Int"), |span: Span| Located::new(Ty::Int, span)),
            map(tag("Unit"), |span: Span| Located::new(Ty::Unit, span)),
            map(in_brackets(ty), |Located { mut content, loc }| {
                content.loc = loc;
                content
            }),
        )),
    )(input)
}
