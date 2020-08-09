mod lexer;
lalrpop_mod!(
    #[allow(unused_imports)]
    parser
);

use thiserror::Error;

use lalrpop_util::{lalrpop_mod, ParseError};

use pijama_ast::node::Block;
use pijama_common::location::{LocatedError, Location};

use lexer::{LexError, Lexer};
use parser::ProgParser;

pub type ParsingResult<T> = Result<T, ParsingError>;
pub type ParsingError = LocatedError<ParsingErrorKind>;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum ParsingErrorKind {
    #[error("Unexpected token \"{found}\", expected {}, ...", expected[..5.min(expected.len())].join(", "))]
    UnexpectedToken {
        found: String,
        expected: Vec<String>,
    },
    #[error("Invalid token")]
    InvalidToken,
    #[error("Extra token")]
    ExtraToken,
    #[error("{0}")]
    Custom(&'static str),
}

pub fn parse(input: &str) -> ParsingResult<Block> {
    let lexer = Lexer::from_input(input);
    let result = ProgParser::new().parse(input, lexer);

    match result {
        Ok(block) => Ok(block),
        Err(err) => Err(match err {
            ParseError::InvalidToken { location } => ParsingError::new(
                ParsingErrorKind::InvalidToken,
                Location::new(location, location),
            ),
            ParseError::UnrecognizedEOF { location, expected } => ParsingError::new(
                ParsingErrorKind::UnexpectedToken {
                    found: "EOF".to_string(),
                    expected,
                },
                Location::new(location, location),
            ),
            ParseError::UnrecognizedToken {
                token: (start, token, end),
                expected,
            } => ParsingError::new(
                ParsingErrorKind::UnexpectedToken {
                    found: token.to_string(),
                    expected,
                },
                Location::new(start, end),
            ),
            ParseError::ExtraToken {
                token: (start, _, end),
            } => ParsingError::new(ParsingErrorKind::ExtraToken, Location::new(start, end)),
            ParseError::User { error } => {
                let msg = match error.content {
                    LexError::Internal => "Unrecognized token",
                    LexError::Custom(msg) => msg,
                };
                ParsingError::new(ParsingErrorKind::Custom(msg), error.loc)
            }
        }),
    }
}
