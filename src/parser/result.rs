use thiserror::Error;

pub type ParseResult<T = ()> = Result<T, ParseError>;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Code ended unexpectedly")]
    UnexpectedEOF,
    #[error("Unclosed delimiter")]
    BracketMismatch,
    #[error("Unexpected character `{0}`")]
    UnexpectedChar(char),
}
