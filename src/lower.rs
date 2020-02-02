mod ctx;
mod scope;

use thiserror::Error;

pub use ctx::Context;

pub type LowerResult<'a, T = ()> = Result<T, LowerError<'a>>;

#[derive(Error, Debug)]
pub enum LowerError<'a> {
    #[error("Undefined atom `{0}`")]
    UndefinedAtom(&'a str),
    #[error("`{0}` requires `{1}` arguments, it received `{2}`")]
    ArityMismatch(&'a str, usize, usize),
    #[error("{0}")]
    Custom(String),
}
