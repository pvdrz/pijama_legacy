//! Pijama's types.
//!
//! This module exposes the `Ty` type which is the type representation used by the
//! type-checker.
use std::fmt;

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
    /// Checks if the index of a `Ty::Var` is contained inside the type.
    pub fn contains(&self, index: usize) -> bool {
        match self {
            Ty::Bool | Ty::Int | Ty::Unit => false,
            Ty::Arrow(ty1, ty2) => ty1.contains(index) || ty2.contains(index),
            Ty::Var(inner) => *inner == index,
        }
    }

    pub fn is_concrete(&self) -> bool {
        match self {
            Ty::Bool | Ty::Int | Ty::Unit => true,
            Ty::Arrow(ty1, ty2) => ty1.is_concrete() && ty2.is_concrete(),
            Ty::Var(_) => false,
        }
    }

    pub fn arity(&self) -> Option<usize> {
        match self {
            Ty::Bool | Ty::Int | Ty::Unit => Some(0),
            Ty::Arrow(ty1, ty2) => {
                ty1.arity()?;
                Some(ty2.arity()? + 1)
            }
            Ty::Var(_) => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Ty> {
        TyIter { ty: Some(self) }
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
            Var(index) => write!(f, "?X{}", index),
        }
    }
}

struct TyIter<'ty> {
    ty: Option<&'ty Ty>,
}

impl<'ty> Iterator for TyIter<'ty> {
    type Item = &'ty Ty;

    fn next(&mut self) -> Option<Self::Item> {
        let ty = self.ty.take()?;
        match ty  {
            Ty::Arrow(t1, t2) => {
                self.ty = Some(t2.as_ref());
                Some(t1.as_ref())
            }
            Ty::Int | Ty::Bool | Ty::Unit | Ty::Var(_) => Some(ty)
        }
    }
}
