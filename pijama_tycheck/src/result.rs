//! Error and Result types related to type-checking.

use thiserror::Error;

use pijama_ast::location::{Located, Location};

use pijama_ty::Ty;

/// The type returned by methods and functions in this module.
pub type TyResult<T = Ty> = Result<T, TyError>;

/// A typing error.
///
/// Each variant here represents a reason why the type-checker could fail.
#[derive(Error, Debug, Eq, PartialEq)]
pub enum TyError {
    /// Variant used when two types that should be equal are not.
    #[error("Type mismatch: expected `{expected}`, found `{found}`")]
    Mismatch { expected: Ty, found: Located<Ty> },
    /// Variant used when a name has not been binded to any type in the current scope.
    #[error("Name `{0}` is not bounded")]
    Unbounded(Located<String>),
}

impl TyError {
    /// Returns the location of the error.
    pub fn loc(&self) -> Location {
        match self {
            TyError::Mismatch { found, .. } => found.loc,
            TyError::Unbounded(name) => name.loc,
        }
    }
}
