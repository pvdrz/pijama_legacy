#![deny(missing_docs)]

//! Crate encapsulating Pijama's AST and 
//! associated types.
pub mod analysis;
pub mod location;
pub mod ty;
pub mod visitor;

use std::fmt::{Debug, Display, Formatter, Result};

use crate::ty::{Ty, TyAnnotation};

pub use location::*;

/// A [`Block`] constitutes a collection of [`Node`]s.
pub type Block<'a> = Vec<Located<Node<'a>>>;

/// Represents the name of a variable or non-primitive
/// function in the AST.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Name<'a>(pub &'a str);

impl<'a> Display for Name<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

/// The different binary operations that Pijama's 
/// syntax supports.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum BinOp {
    /// Add operation.
    Add,
    /// Subtract operation.
    Sub,
    /// Multiply operation.
    Mul,
    /// Divide operation.
    Div,
    /// Remainder/Modulo operation.
    Rem,
    /// Logical And operation.
    And,
    /// Logical Or operation.
    Or,
    /// Bit-wise And operation.
    BitAnd,
    /// Bit-wise Or operation.
    BitOr,
    /// Bit-wise Xor operation.
    BitXor,
    /// Right shift operation.
    Shr,
    /// Left shift operation.
    Shl,
    /// Equality operation.
    Eq,
    /// Not Equal operation.
    Neq,
    /// Less Than operation.
    Lt,
    /// Greater Than operation.
    Gt,
    /// Less Than Or Equal operation.
    Lte,
    /// Greater Than Or Equal operation.
    Gte,
}

impl<'a> Display for BinOp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use BinOp::*;
        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Div => write!(f, "/"),
            Rem => write!(f, "%"),
            And => write!(f, "&&"),
            Or => write!(f, "||"),
            BitAnd => write!(f, "&"),
            BitOr => write!(f, "|"),
            BitXor => write!(f, "^"),
            Shr => write!(f, ">>"),
            Shl => write!(f, "<<"),
            Eq => write!(f, "=="),
            Neq => write!(f, "!="),
            Lt => write!(f, "<"),
            Gt => write!(f, ">"),
            Lte => write!(f, "<="),
            Gte => write!(f, ">="),
        }
    }
}

/// The unary operators that Pijama's syntax supports.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum UnOp {
    /// Numeric Negation operation.
    Neg,
    /// Logical Negation operation 
    Not,
}

impl<'a> Display for UnOp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use UnOp::*;
        match self {
            Not => write!(f, "!"),
            Neg => write!(f, "-"),
        }
    }
}

/// The literal types that Pijama's syntax supports.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Literal {
    /// Boolean Literal.
    Bool(bool),
    /// Unit Literal.
    Unit,
    /// Numeric Literal.
    Number(i64),
}

impl From<i64> for Literal {
    fn from(n: i64) -> Self {
        Literal::Number(n)
    }
}

impl From<bool> for Literal {
    fn from(b: bool) -> Self {
        Literal::Bool(b)
    }
}

impl<'a> Display for Literal {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use Literal::*;
        match self {
            Bool(b) => write!(f, "{}", b),
            Unit => write!(f, "unit"),
            Number(num) => write!(f, "{}", num),
        }
    }
}

/// The primitives that Pijama's syntax supports.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Primitive {
    /// Built-in Print primitive.
    Print,
}

impl<'a> Display for Primitive {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use Primitive::*;

        match self {
            Print => write!(f, "print"),
        }
    }
}

/// Encapsulates a conditional statement in Pijama's syntax of the 
/// form "if `cond` then `body`".
#[derive(Debug, Eq, PartialEq)]
pub struct Branch<'a> {
    /// The conditional part of the Branch that is checked for truthiness.
    pub cond: Located<Block<'a>>,
    /// The body of the Branch that is executed if `cond` is true.
    pub body: Located<Block<'a>>,
}

/// A [`Node`] in the AST that encapsulates the different
/// expressions and statements that Pijama's syntax supports.
#[derive(Debug, Eq, PartialEq)]
pub enum Node<'a> {
    /// Expression containing a binary operation.
    BinaryOp(BinOp, Box<Located<Node<'a>>>, Box<Located<Node<'a>>>),
    /// Expression containing a unary operator.
    UnaryOp(UnOp, Box<Located<Node<'a>>>),
    /// Statement containing a Let binding.
    LetBind(
        TyAnnotation<Name<'a>>,
        Box<Located<Node<'a>>>,
    ),
    /// Expression containing a conditional.
    Cond(Branch<'a>, Vec<Branch<'a>>, Located<Block<'a>>),
    /// Statement containing a Function Definition.
    FnDef(
        Option<Located<Name<'a>>>,
        Vec<TyAnnotation<Name<'a>>>,
        Located<Block<'a>>,
        Located<Ty>,
    ),
    /// Expression containing a Function Call.
    Call(Box<Located<Node<'a>>>, Block<'a>),
    /// Expression containing a Literal.
    Literal(Literal),
    /// Expression containing a Name.
    Name(Name<'a>),
    /// Expression containing a Primitive function.
    PrimFn(Primitive),
}
