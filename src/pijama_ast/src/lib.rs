pub mod analysis;
pub mod location;
pub mod ty;
pub mod visitor;

use std::fmt::{Debug, Display, Formatter, Result};

use crate::ty::{Ty, TyAnnotation};

pub use location::*;

pub type Block<'a> = Vec<Located<Node<'a>>>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Name<'a>(pub &'a str);

impl<'a> Display for Name<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    Shr,
    Shl,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum UnOp {
    Neg,
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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Literal {
    Bool(bool),
    Unit,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Primitive {
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

/// Encapsulates a conditional where "if `cond` then `body`"
#[derive(Debug, Eq, PartialEq)]
pub struct Branch<'a> {
    pub cond: Located<Block<'a>>,
    pub body: Located<Block<'a>>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Node<'a> {
    BinaryOp(BinOp, Box<Located<Node<'a>>>, Box<Located<Node<'a>>>),
    UnaryOp(UnOp, Box<Located<Node<'a>>>),
    LetBind(
        Located<Name<'a>>,
        Option<Located<Ty>>,
        Box<Located<Node<'a>>>,
    ),
    Cond(Branch<'a>, Vec<Branch<'a>>, Located<Block<'a>>),
    FnDef(
        Option<Located<Name<'a>>>,
        Vec<Located<TyAnnotation<'a>>>,
        Located<Block<'a>>,
        Option<Located<Ty>>,
    ),
    Call(Box<Located<Node<'a>>>, Block<'a>),
    Literal(Literal),
    Name(Name<'a>),
    PrimFn(Primitive),
}
