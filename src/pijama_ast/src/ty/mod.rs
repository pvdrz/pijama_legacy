//! The AST representation of types.

/// A type in the AST.
///
/// This type must only represent the kinds of types is possible to write in the AST. Other `Ty`
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

/// A type annotation.
///
/// This represents an annotation of a `Name` with a type and is used to represent any type
/// annotations written by the user.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TyAnnotation<'a> {
    pub name: Name<'a>,
    pub ty: Ty,
}
