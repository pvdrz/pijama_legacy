use thiserror::Error;

use lalrpop_util::{lalrpop_mod, ParseError};

use pijama_ast::{
    location::{Located, Location},
    node::Block,
};

mod lexer;
lalrpop_mod!(
    #[allow(unused_imports)]
    parser
);

use lexer::{LexError, Lexer, Token};
use parser::ProgParser;

#[derive(Error, Debug, Eq, PartialEq)]
#[error("{kind}")]
pub struct ParsingError {
    loc: Location,
    kind: ParsingErrorKind,
}

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

impl ParsingError {
    pub fn loc(&self) -> Location {
        self.loc
    }
}

impl<'a> From<ParseError<usize, Token<'a>, Located<LexError>>> for ParsingError {
    fn from(error: ParseError<usize, Token<'a>, Located<LexError>>) -> Self {
        match error {
            ParseError::InvalidToken { location } => ParsingError {
                loc: Location::new(location, location),
                kind: ParsingErrorKind::InvalidToken,
            },
            ParseError::UnrecognizedEOF { location, expected } => ParsingError {
                loc: Location::new(location, location),
                kind: ParsingErrorKind::UnexpectedToken {
                    found: "EOF".to_string(),
                    expected,
                },
            },
            ParseError::UnrecognizedToken {
                token: (start, token, end),
                expected,
            } => ParsingError {
                loc: Location::new(start, end),
                kind: ParsingErrorKind::UnexpectedToken {
                    found: token.to_string(),
                    expected,
                },
            },
            ParseError::ExtraToken {
                token: (start, _, end),
            } => ParsingError {
                loc: Location::new(start, end),
                kind: ParsingErrorKind::ExtraToken,
            },
            ParseError::User { error } => {
                let msg = match error.content {
                    LexError::Internal => "Unrecognized token",
                    LexError::Custom(msg) => msg,
                };
                ParsingError {
                    loc: error.loc,
                    kind: ParsingErrorKind::Custom(msg),
                }
            }
        }
    }
}

pub fn parse(input: &str) -> Result<Block, ParsingError> {
    let lexer = Lexer::from_input(input);
    let result = ProgParser::new().parse(input, lexer);

    match result {
        Ok(block) => Ok(block),
        Err(err) => Err(err.into()),
    }
}
