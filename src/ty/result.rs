use thiserror::Error;

use crate::{
    ast::{Located, Location},
    ty::Ty,
};

pub type TyResult<T = Ty> = Result<T, TyError>;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum TyError {
    #[error("Unexpected type: expected {expected}, found {found}")]
    Unexpected { expected: Ty, found: Located<Ty> },
    #[error("Name {0} is not bounded")]
    Unbound(Located<String>),
    #[error("Unexpected type: expected function, found {0}")]
    ExpectedFn(Located<Ty>),
    #[error("Unexpected type: expected a basic type, found {0}")]
    ExpectedBasic(Located<Ty>),
    #[error("Missing type: type cannot be inferred")]
    Missing(Located<()>),
}

impl TyError {
    pub fn loc(&self) -> Location {
        match self {
            TyError::Unexpected { found, .. } => found.loc,
            TyError::Unbound(name) => name.loc,
            TyError::ExpectedBasic(ty) | TyError::ExpectedFn(ty) => ty.loc,
            TyError::Missing(unit) => unit.loc,
        }
    }
}
