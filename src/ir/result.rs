use thiserror::Error;

pub type LowerResult<T = ()> = Result<T, LowerError>;

#[derive(Error, Debug)]
pub enum LowerError {
    #[error("Malformed expression")]
    MalformedExpr,
    #[error("Missing expression")]
    MissingExpr,
    #[error("Not an atom")]
    NotAtom,
    #[error("Not a sequence")]
    NotSeq,
}
