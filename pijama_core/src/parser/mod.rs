//! The Pijama parser.
//!
//! Each element in the [`ast`] module has a corresponding module here. For example, the parsers
//! for the [`Literal`] type are inside the [`literal`] submodule. Each module might include
//! intermediate parsers that deserve their own explanation.
//!
//! The documentation of some submodules includes a simplified [ABNF] grammar to explain the
//! expressions it is able to parse. These grammars never include details about spaces, those are
//! discussed in the documentation of each parser.
//!
//! The main entry point of this module is the [`parse`] function, which parses the source code in
//! a string slice as a [`Block`].
//!
//! The whole parser is written in nom, if you have any doubts about the behavior of certain
//! parsing combinators after reading this documentation, the [nom docs] are a good place to start.
//!
//! [`ast`]: crate::ast
//! [`Literal`]: crate::ast::Literal
//! [`literal`]: crate::parser::literal
//! [`Block`]: crate::ast::Block
//!
//! [ABNF]: https://en.wikipedia.org/wiki/Augmented_Backus–Naur_form
//! [nom docs]: https://docs.rs/nom/
use thiserror::Error;

use nom::{
    character::complete::multispace0,
    combinator::all_consuming,
    error::{ErrorKind, ParseError},
    lib::std::fmt::Formatter,
    Err::*,
};

use pijama_ast::{Block, Located, Span};

use block::block0;
use helpers::surrounded;

use std::{fmt, fmt::Display};

mod bin_op;
mod block;
mod helpers;
mod literal;
mod name;
mod node;
mod primitive;
mod ty;
mod un_op;

type IResult<'a, T> = nom::IResult<Span<'a>, T, ParsingError<'a>>;

/// Produces a [`Block`] from a string slice.
///
/// This function fails if the whole string is not consumed during parsing or if there is an error
/// with the inner parsers.
pub fn parse(input: &str) -> Result<Located<Block>, ParsingError<'_>> {
    let span = Span::new(input);
    let result: IResult<Located<Block>> = all_consuming(surrounded(block0, multispace0))(span);
    match result {
        Ok((_, block)) => Ok(block),
        Err(Error(e)) | Err(Failure(e)) => Err(e),
        _ => unreachable!(),
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
pub struct ParsingError<'a> {
    pub span: Span<'a>,
    kind: ErrorKind,
    context: Option<String>,
}

impl<'a> Display for ParsingError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Parsing error: ")?;
        match self.context.as_ref() {
            Some(context) => write!(f, "{}", context),
            None => write!(f, "Parsing rule `{:?}` failed.", self.kind),
        }
    }
}

impl<'a> ParsingError<'a> {
    pub fn with_context(_: Span<'a>, context: String, other: Self) -> Self {
        ParsingError {
            span: other.span,
            kind: other.kind,
            context: Some(context),
        }
    }
}

impl<'a> ParseError<Span<'a>> for ParsingError<'a> {
    fn from_error_kind(span: Span<'a>, kind: ErrorKind) -> Self {
        ParsingError {
            span,
            kind,
            context: None,
        }
    }

    fn append(_: Span<'a>, _: ErrorKind, other: Self) -> Self {
        other
    }

    fn from_char(span: Span<'a>, c: char) -> Self {
        ParsingError {
            span,
            kind: ErrorKind::Char,
            context: Some(format!("Expected character '{}'.", c)),
        }
    }

    fn add_context(_: Span<'a>, context: &'static str, other: Self) -> Self {
        ParsingError {
            span: other.span,
            kind: other.kind,
            context: Some(context.to_string()),
        }
    }
}
