use thiserror::Error;

pub type ParseResult<T = ()> = Result<T, ParseError>;

#[derive(Error, Debug)]
#[error("{kind} while parsing at location {loc}")]
pub struct ParseError {
    kind: ParseErrorKind,
    loc: usize,
}

impl ParseError {
    pub fn new(loc: usize, kind: ParseErrorKind) -> Self {
        ParseError { kind, loc }
    }
}

#[derive(Error, Debug)]
pub enum ParseErrorKind {
    #[error("Code ended unexpectedly")]
    UnexpectedEOF,
    #[error("Unclosed delimiter")]
    BracketMismatch,
    #[error("Unexpected character `{0}`")]
    UnexpectedChar(char),
}
