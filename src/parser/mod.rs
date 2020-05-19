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

use nom::{character::complete::multispace0, combinator::all_consuming, error::ErrorKind, Err::*};

use crate::{
    ast::{Block, Span},
    LangResult,
};

use block::block0;
use helpers::surrounded;

mod bin_op;
mod block;
mod helpers;
mod literal;
mod name;
mod node;
mod ty;
mod un_op;

type IResult<'a, T> = nom::IResult<Span<'a>, T, (Span<'a>, ErrorKind)>;

/// Produces a [`Block`] from a string slice.
///
/// This function fails if the whole string is not consumed during parsing or if there is an error
/// with the inner parsers.
pub fn parse(input: &str) -> LangResult<Block> {
    let span = Span::new(input);
    let result: IResult<Block> = all_consuming(surrounded(block0, multispace0))(span);
    match result {
        Ok((_, block)) => Ok(block),
        Err(Error(e)) | Err(Failure(e)) => Err(ParseError {
            span: e.0,
            kind: e.1,
        })?,
        _ => unreachable!(),
    }
}

#[derive(Error, Debug)]
#[error("Parsing rule `{kind:?}` failed.")]
pub struct ParseError<'a> {
    pub span: Span<'a>,
    kind: ErrorKind,
}
