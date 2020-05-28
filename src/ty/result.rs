use thiserror::Error;

use pijama_ast::{Located, Location};

use crate::ty::Ty;

/// The type returned by methods and functions in this module.
pub type TyResult<T = Ty> = Result<T, TyError>;

/// A typing error.
///
/// Each variant here represents a reason why the type-checker could fail.
#[derive(Error, Debug, Eq, PartialEq)]
pub enum TyError {
    /// Variant used when two types that should be equal are not.
    #[error("Unexpected type: expected `{expected}`, found `{found}`")]
    Unexpected { expected: Ty, found: Located<Ty> },
    /// Variant used when a name has not been binded to any type in the current scope.
    #[error("Name `{0}` is not bounded")]
    Unbound(Located<String>),
    /// Variant used when a type was expected to be a `Ty::Arrow` function type.
    #[error("Unexpected type: expected function, found `{0}`")]
    ExpectedFn(Located<Ty>),
    /// Variant used when a type was expected to not be a `Ty::Arrow` function type.
    #[error("Unexpected type: expected a basic type, found `{0}`")]
    ExpectedBasic(Located<Ty>),
    /// Variant used when a required type annotation is missing.
    #[error("Missing type: type cannot be inferred")]
    Missing(Located<()>),
}

impl TyError {
    /// Returns the location of the error.
    pub fn loc(&self) -> Location {
        match self {
            TyError::Unexpected { found, .. } => found.loc,
            TyError::Unbound(name) => name.loc,
            TyError::ExpectedBasic(ty) | TyError::ExpectedFn(ty) => ty.loc,
            TyError::Missing(unit) => unit.loc,
        }
    }
}
