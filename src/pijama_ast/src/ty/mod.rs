//! The AST representation of types.
use crate::Name;

/// A type in the AST.
///
/// This type must only represent the kinds of types that Pijama's AST can represent. Other `Ty`
/// types exist with different purposes.
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

/// A type annotation.
///
/// This represents an annotation of a `Name` with a type and is used to represent any type
/// annotations written by the user.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TyAnnotation<'a> {
    pub name: Name<'a>,
    pub ty: Ty,
}
