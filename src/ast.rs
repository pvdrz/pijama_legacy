use std::fmt;

use crate::ty::{Binding, Ty};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Name<'a>(pub &'a str);
impl<'a> fmt::Display for Name<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BinOp {
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

impl<'a> fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BinOp::*;
        match self {
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Times => write!(f, "*"),
            Divide => write!(f, "/"),
            Modulo => write!(f, "%"),
            And => write!(f, "&&"),
            Or => write!(f, "||"),
            BitAnd => write!(f, "&"),
            BitOr => write!(f, "|"),
            BitXor => write!(f, "^"),
            Equal => write!(f, "=="),
            NotEqual => write!(f, "!="),
            LessThan => write!(f, "<"),
            GreaterThan => write!(f, ">"),
            LessThanOrEqual => write!(f, "<="),
            GreaterThanOrEqual => write!(f, ">="),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnOp {
    Minus,
    Not,
}

impl<'a> fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use UnOp::*;
        match self {
            Not => write!(f, "!"),
            Minus => write!(f, "-"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Literal {
    Bool(bool),
    Unit,
    Number(i128),
}

impl Into<Literal> for i128 {
    fn into(self) -> Literal {
        Literal::Number(self)
    }
}

impl Into<Literal> for bool {
    fn into(self) -> Literal {
        Literal::Bool(self)
    }
}

impl<'a> fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Literal::*;
        match self {
            Bool(b) => write!(f, "{}", b),
            Unit => write!(f, "unit"),
            Number(num) => write!(f, "{}", num),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Node<'a> {
    BinaryOp(BinOp, Box<Node<'a>>, Box<Node<'a>>),
    UnaryOp(UnOp, Box<Node<'a>>),
    LetBind(Name<'a>, Option<Ty>, Box<Node<'a>>),
    Cond(Vec<Node<'a>>, Vec<Node<'a>>, Vec<Node<'a>>),
    FnDef(Name<'a>, Vec<Binding<'a>>, Vec<Node<'a>>, Option<Ty>),
    FnRecDef(Name<'a>, Vec<Binding<'a>>, Vec<Node<'a>>, Ty),
    Call(Name<'a>, Vec<Node<'a>>),
    Literal(Literal),
    Name(Name<'a>),
}
