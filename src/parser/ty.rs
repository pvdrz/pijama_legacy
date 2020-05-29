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
//! In addition we have the [`binding`] parser, which parses expressions with the grammar
//!
//! ```abnf
//! binding = name ":" ty
//! ```
//! The parser for names is explained in the [`name`] module.
//!
//! [`ty`]: crate::parser::ty::ty
//! [`base_ty`]: crate::parser::ty::base_ty
//! [`binding`]: crate::parser::ty::binding
//! [`Ty`]: crate::ty::Ty
//! [`Binding`]: crate::ty::Binding
//! [`name`]: crate::parser::name
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, space0},
    combinator::{map, opt},
    sequence::{pair, preceded},
};

use pijama_ast::{
    ty::{Binding, Ty},
    Located, Location,
};

use crate::parser::{
    helpers::{in_brackets, surrounded, with_context},
    name::name,
    IResult, Span,
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
            Located::new(
                Ty::Arrow(Box::new(t1.content), Box::new(t2.content)),
                Location::new(t1.loc.start, t2.loc.end),
            ),
        ))
    } else {
        Ok((rem, t1))
    }
}

/// Parser for type bindings.
///
/// This parser returns a [`Binding`], there can be any number of spaces surrounding the `:`,
/// including no spaces at all.
pub fn binding(input: Span) -> IResult<Located<Binding>> {
    map(
        pair(name, colon_ty),
        |(
            Located {
                content: name,
                loc: loc1,
            },
            Located {
                content: ty,
                loc: loc2,
            },
        )| { Located::new(Binding { name, ty }, loc1 + loc2) },
    )(input)
}

/// Parses types preceded by a colon.
///
/// This parser returns a [`Ty`] and there can be any number of spaces surrounding the colon.
///
/// This parser exists with the sole purpose of being reutilized for type bindings that are not
/// stored in [`Binding`]s such as the return type of functions or the optional type binding for
/// let bindings.
pub fn colon_ty(input: Span) -> IResult<Located<Ty>> {
    preceded(surrounded(char(':'), space0), ty)(input)
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
