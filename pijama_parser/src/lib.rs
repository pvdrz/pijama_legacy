use thiserror::Error;

use lalrpop_util::{lalrpop_mod, ParseError};

use pijama_ast::{
    location::{Located, Location},
    node::Block,
};

mod lexer;
lalrpop_mod!(#[allow(unused_imports)] parser);

use lexer::{LexError, Lexer, Token};
use parser::ProgParser;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum ParsingError {
    #[error("Unexpected token")]
    UnexpectedToken {
        loc: Location,
        expected: Vec<String>,
    },
    #[error("{msg}")]
    Custom { loc: Location, msg: &'static str },
}

impl ParsingError {
    pub fn loc(&self) -> Location {
        match self {
            ParsingError::UnexpectedToken { loc, .. } | ParsingError::Custom { loc, .. } => *loc,
        }
    }
}

impl<'a> From<ParseError<usize, Token<'a>, Located<LexError>>> for ParsingError {
    fn from(error: ParseError<usize, Token<'a>, Located<LexError>>) -> Self {
        match error {
            ParseError::InvalidToken { location } => ParsingError::UnexpectedToken {
                loc: Location::new(location, location),
                expected: Vec::default(),
            },
            ParseError::UnrecognizedEOF { location, expected } => ParsingError::UnexpectedToken {
                loc: Location::new(location, location),
                expected,
            },
            ParseError::UnrecognizedToken {
                token: (start, _, end),
                expected,
            } => ParsingError::UnexpectedToken {
                loc: Location::new(start, end),
                expected,
            },
            ParseError::ExtraToken {
                token: (start, _, end),
            } => ParsingError::UnexpectedToken {
                loc: Location::new(start, end),
                expected: Vec::default(),
            },
            ParseError::User { error } => {
                let msg = match error.content {
                    LexError::Internal => "Internal lexing error",
                    LexError::Custom(msg) => msg,
                };
                ParsingError::Custom {
                    msg,
                    loc: error.loc,
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
