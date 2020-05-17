//! The Pijama parser.
//!
//! Each element in the [`ast`](crate::ast) module has a corresponding module here. For example,
//! the parsers for the [`Literal`](crate::ast::Literal) type are inside the
//! [`literal`](crate::parser::literal) submodule. Each module might include intermediate parsers
//! that deserve their own explanation.
//!
//! The documentation of some submodules includes a simplified
//! [ABNF](https://en.wikipedia.org/wiki/Augmented_Backus–Naur_form) grammar to explain the
//! expressions it is able to parse. These grammars never include details about spaces, those are
//! discussed in the documentation of each parser.
//!
//! The main entry point of this module is the [`parse`](crate::parser::parse) function, which
//! parses the source code in a string slice as a [`Block`](crate::ast::Block).
//!
//! The whole parser is written in nom, if you have any doubts about the behavior of certain
//! parsing combinators after reading this documentation, the [nom
//! documentation](https://docs.rs/nom/5.1.1/nom/) is a good place to start.

use nom::{
    error::{convert_error, VerboseError},
    Err::{Error, Failure},
    IResult,
};

use nom::{character::complete::multispace0, combinator::all_consuming};

use crate::ast::Block;
use crate::{LangError, LangResult};

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

pub fn parse<'a>(input: &'a str) -> LangResult<Block<'a>> {
    let result: IResult<&str, Block, VerboseError<&str>> =
        all_consuming(surrounded(block0, multispace0))(input);
    match result {
        Ok((_, block)) => Ok(block),
        Err(Error(e)) | Err(Failure(e)) => Err(LangError::Parse(convert_error(input, e))),
        _ => Err(LangError::Parse(String::new())),
    }
}
