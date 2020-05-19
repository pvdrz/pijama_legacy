use std::fmt;

use crate::ty::{Binding, Ty};

use either::Either;
use lazy_static::lazy_static;

use std::collections::HashMap;

pub type Block<'a> = Vec<Node<'a>>;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Name<'a>(pub &'a str);
impl<'a> fmt::Display for Name<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
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

impl<'a> fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnOp {
    Neg,
    Not,
}

impl<'a> fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use UnOp::*;
        match self {
            Not => write!(f, "!"),
            Neg => write!(f, "-"),
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BuiltInFn {
    Print,
}

lazy_static! {
    pub static ref BUILT_IN_FNS: HashMap<&'static str, BuiltInFn> = {
        let mut m = HashMap::new();
        m.insert("print", BuiltInFn::Print);
        m
    };
}

impl fmt::Display for BuiltInFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuiltInFn::Print => write!(f, "print"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Node<'a> {
    BinaryOp(BinOp, Box<Node<'a>>, Box<Node<'a>>),
    UnaryOp(UnOp, Box<Node<'a>>),
    LetBind(Name<'a>, Option<Ty>, Box<Node<'a>>),
    Cond(Block<'a>, Block<'a>, Block<'a>),
    FnDef(Option<Name<'a>>, Vec<Binding<'a>>, Block<'a>, Option<Ty>),
    FnRecDef(Name<'a>, Vec<Binding<'a>>, Block<'a>, Ty),
    Call(Either<Name<'a>, BuiltInFn>, Block<'a>),
    Literal(Literal),
    Name(Name<'a>),
}
