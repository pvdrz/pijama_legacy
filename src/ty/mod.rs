//! Types and functions related to Pijama's type system.
//!
//! The entry point for this module is the `ty_check` function which takes care of type inference
//! and checking.

use std::fmt;
use pijama_ast::Name;

mod result;
mod ty_check;

pub use result::{TyError, TyResult};
pub use ty_check::{expect_ty, ty_check};

/// The type of a term.
///
/// Each variant here represents the type a term might have.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    /// The type of booleans.
    Bool,
    /// The type of (signed) integers.
    Int,
    /// The [unit type](https://en.wikipedia.org/wiki/Unit_type).
    Unit,
    /// The type of functions between two types.
    Arrow(Box<Ty>, Box<Ty>),
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Ty::*;
        match self {
            Bool => write!(f, "Bool"),
            Int => write!(f, "Int"),
            Unit => write!(f, "Unit"),
            Arrow(t1, t2) => {
                if let Arrow(_, _) = t1.as_ref() {
                    write!(f, "({}) -> {}", t1, t2)
                } else {
                    write!(f, "{} -> {}", t1, t2)
                }
            }
        }
    }
}

/// A type binding.
///
/// This represents a binding of a `Name` to a type and is used inside the type checker as the
/// default way of encoding that a variable has a type in the current scope.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Binding<'a> {
    pub name: Name<'a>,
    pub ty: Ty,
}
