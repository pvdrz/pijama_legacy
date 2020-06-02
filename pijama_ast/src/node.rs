//! Nodes of Pijama's AST.
//!
//! The purpose of the types in this module is to represent the syntax of Pijama as faithfully as
//! possible. Some types here are used through Pijama's different internal representations.
use std::{
    collections::VecDeque,
    fmt::{Debug, Display, Formatter, Result},
};

use crate::{
    location::{Located, Location},
    ty::TyAnnotation,
};

/// A block is a sequence of nodes terminating in an expression.
#[derive(Debug, Eq, PartialEq)]
pub struct Block<'a> {
    /// Nodes of the block.
    pub nodes: VecDeque<Node<'a>>,
    /// Terminating expression of the block.
    pub expr: Box<Located<Expression<'a>>>,
}

/// A Node in the AST that encapsulates the different expressions and statements that Pijama's
/// syntax supports.
#[derive(Debug, Eq, PartialEq)]
pub enum Node<'a> {
    /// A statement.
    Stat(Located<Statement<'a>>),
    /// An expression.
    Expr(Located<Expression<'a>>),
}

impl<'a> Node<'a> {
    /// Return the location of the node.
    pub fn loc(&self) -> Location {
        match *self {
            Node::Stat(Located { loc, .. }) | Node::Expr(Located { loc, .. }) => loc,
        }
    }
}

/// An AST node that performs an action.
#[derive(Debug, Eq, PartialEq)]
pub enum Statement<'a> {
    /// Statement containing an assignment.
    Assign(TyAnnotation<Located<Name<'a>>>, Located<Expression<'a>>),
    /// Statement containing a function definition.
    FnDef(
        Located<Name<'a>>,
        Vec<TyAnnotation<Located<Name<'a>>>>,
        TyAnnotation<Block<'a>>,
    ),
}

/// An AST node that produces a value.
#[derive(Debug, Eq, PartialEq)]
pub enum Expression<'a> {
    /// Expression containing a binary operation.
    BinaryOp(
        BinOp,
        Box<Located<Expression<'a>>>,
        Box<Located<Expression<'a>>>,
    ),
    /// Expression containing a unary operation.
    UnaryOp(UnOp, Box<Located<Expression<'a>>>),
    /// Expression containing a conditional.
    Cond(Branch<'a>, Vec<Branch<'a>>, Block<'a>),
    /// Expression containing an anonymous function.
    AnonFn(
        Vec<TyAnnotation<Located<Name<'a>>>>,
        TyAnnotation<Block<'a>>,
    ),
    /// Expression containing a function call.
    Call(Box<Located<Expression<'a>>>, Vec<Located<Expression<'a>>>),
    /// Expression containing a literal.
    Literal(Literal),
    /// Expression containing a name.
    Name(Name<'a>),
    /// Expression containing a primitive function.
    PrimFn(Primitive),
}

/// Encapsulates a conditional statement in Pijama's syntax. It is used to represent both `if` and
/// `elif` branches.
#[derive(Debug, Eq, PartialEq)]
pub struct Branch<'a> {
    /// The condition of the branch.
    pub cond: Block<'a>,
    /// The body of the branch that is evaluated if the condition is true.
    pub body: Block<'a>,
}

/// Represents the name of a variable or non-primitive function in the AST.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Name<'a>(pub &'a str);

impl<'a> Display for Name<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

/// The different binary operators that Pijama's syntax supports.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum BinOp {
    /// Addition operator.
    Add,
    /// Subtraction operator.
    Sub,
    /// Multiplication operator.
    Mul,
    /// Division operator.
    Div,
    /// Remainder/Modulo operator.
    Rem,
    /// Logical And operator.
    And,
    /// Logical Or operator.
    Or,
    /// Bitwise And operator.
    BitAnd,
    /// Bitwise Or operator.
    BitOr,
    /// Bitwise Xor operator.
    BitXor,
    /// Right-shift operator.
    Shr,
    /// Left-shift operator.
    Shl,
    /// Equality operator.
    Eq,
    /// Not Equal operator.
    Neq,
    /// Less Than operator.
    Lt,
    /// Greater Than operator.
    Gt,
    /// Less Than Or Equal operator.
    Lte,
    /// Greater Than Or Equal operator.
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
    /// Arithmetic Negation operator.
    Neg,
    /// Logical Negation operator.
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

/// The literal values that Pijama's syntax supports.
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
