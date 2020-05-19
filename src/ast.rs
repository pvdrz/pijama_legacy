use std::fmt::{self, Debug, Display, Formatter};

use crate::ty::{Binding, Ty};

pub type Block<'a> = Vec<Located<Node<'a>>>;

#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}

impl Location {
    pub fn new(start: usize, end: usize) -> Self {
        Location { start, end }
    }
}

impl std::ops::Add for Location {
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        self.end = other.end;
        self
    }
}

#[derive(Debug)]
pub struct Located<T: Debug> {
    pub content: T,
    pub loc: Location,
}

impl<T: Debug> Located<T> {
    pub fn new(content: T, loc: impl Into<Location>) -> Self {
        Located {
            content,
            loc: loc.into(),
        }
    }
}

impl<T: Display + Debug> Display for Located<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl<T: Eq + Debug> Eq for Located<T> {}

impl<T: PartialEq + Debug> PartialEq for Located<T> {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Name<'a>(pub &'a str);

impl<'a> Display for Name<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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

impl<'a> Display for BinOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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

impl<'a> Display for UnOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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

impl<'a> Display for Literal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use Literal::*;
        match self {
            Bool(b) => write!(f, "{}", b),
            Unit => write!(f, "unit"),
            Number(num) => write!(f, "{}", num),
        }
    }
}

#[derive(Debug)]
pub enum Node<'a> {
    BinaryOp(BinOp, Box<Located<Node<'a>>>, Box<Located<Node<'a>>>),
    UnaryOp(UnOp, Box<Located<Node<'a>>>),
    LetBind(
        Located<Name<'a>>,
        Option<Located<Ty>>,
        Box<Located<Node<'a>>>,
    ),
    Cond(Located<Block<'a>>, Located<Block<'a>>, Located<Block<'a>>),
    FnDef(
        Option<Located<Name<'a>>>,
        Vec<Located<Binding<'a>>>,
        Located<Block<'a>>,
        Option<Located<Ty>>,
    ),
    FnRecDef(
        Located<Name<'a>>,
        Vec<Located<Binding<'a>>>,
        Located<Block<'a>>,
        Located<Ty>,
    ),
    Call(Located<Name<'a>>, Block<'a>),
    Literal(Literal),
    Name(Name<'a>),
}
