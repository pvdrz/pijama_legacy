//! Error and Result types related to type-checking.

use thiserror::Error;

use pijama_common::location::Location;

use pijama_ty::Ty;

/// The type returned by methods and functions in this module.
pub type TyResult<T = Ty> = Result<T, TyError>;

/// A kind of typing error.
///
/// Each variant here represents a reason why the type-checker could fail.
#[derive(Error, Debug, Eq, PartialEq)]
pub enum TyErrorKind {
    /// Variant used when two types that should be equal are not.
    #[error("Type mismatch: expected `{expected}`, found `{found}`")]
    Mismatch { expected: Ty, found: Ty },
    /// Variant used when a name has not been binded to any type in the current scope.
    #[error("Local `{0}` is not bounded")]
    Unbounded(String),
}

/// A typing error.
#[derive(Error, Debug)]
#[error("`{kind}`")]
pub struct TyError {
    kind: TyErrorKind,
    loc: Location,
}

impl TyError {
    pub fn new(kind: TyErrorKind, loc: Location) -> Self {
        Self { kind, loc }
    }

    pub fn loc(&self) -> Location {
        self.loc
    }
}

impl PartialEq for TyError {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl Eq for TyError {}
