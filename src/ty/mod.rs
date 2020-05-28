use std::fmt;

use pijama_ast::ty::Ty as TyAST;

mod result;
mod ty_check;

pub use result::{TyError, TyResult};
pub use ty_check::ty_check;

/// A type used by the type-checker.
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
    /// Type variable, used for unification.
    Var(usize),
}

impl Ty {
    /// Checks if a `Ty::Var` is contained inside the type.
    ///
    /// This method does not work with other `Ty` variants and must be used only with `Ty:Var`.
    fn contains(&self, ty: &Ty) -> bool {
        match self {
            Ty::Bool | Ty::Int | Ty::Unit => false,
            Ty::Arrow(ty1, ty2) => ty1.contains(ty) || ty2.contains(ty),
            Ty::Var(_) => self == ty,
        }
    }
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
            Var(index) => write!(f, "T{}", index),
        }
    }
}

impl From<TyAST> for Ty {
    fn from(ty_ast: TyAST) -> Self {
        match ty_ast {
            TyAST::Bool => Ty::Bool,
            TyAST::Int => Ty::Int,
            TyAST::Unit => Ty::Unit,
            TyAST::Arrow(t1, t2) => Ty::Arrow(Box::new(Ty::from(*t1)), Box::new(Ty::from(*t2))),
        }
    }
}
