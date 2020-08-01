//! The AST representation of types.
use std::fmt::Debug;

use pijama_common::location::Located;

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
    /// A missing type. Used when an item in the AST did not have a type annotation.
    Missing,
}

/// A type annotation.
///
/// This represents an annotation of an AST item with a type and is used to represent any type
/// annotations written by the user.
#[derive(Debug, Eq, PartialEq)]
pub struct TyAnnotation<I: Debug + Eq + PartialEq> {
    /// The annotated item.
    pub item: I,
    /// The type specified by the annotation.
    pub ty: Located<Ty>,
}
