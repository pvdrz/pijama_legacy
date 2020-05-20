use std::fmt::{Debug, Display, Formatter, Result};

use crate::ty::{Binding, Ty};

mod location;
mod visitor;

pub use location::*;

use visitor::NodeVisitor;

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
    fn fmt(&self, f: &mut Formatter) -> Result {
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
    Call(Box<Located<Node<'a>>>, Block<'a>),
    Literal(Literal),
    Name(Name<'a>),
}

pub struct RecursionChecker<'a> {
    name: Name<'a>,
    is_rec: bool,
}

impl<'a> RecursionChecker<'a> {
    pub fn run(name: Name<'a>, body: &Block<'a>) -> bool {
        let mut this = RecursionChecker {
            name,
            is_rec: false,
        };
        this.visit_block(body);
        this.is_rec
    }
}

impl<'a> NodeVisitor<'a> for RecursionChecker<'a> {
    fn visit_name(&mut self, name: &Name<'a>) {
        if *name == self.name {
            self.is_rec = true;
        } else {
            self.super_name(name);
        }
    }

    fn visit_let_bind(
        &mut self,
        name: &Located<Name<'a>>,
        opt_ty: &Option<Located<Ty>>,
        body: &Located<Node<'a>>,
    ) {
        // check if function name is no being shadowed
        if name.content != self.name {
            // if is not, keep visiting.
            self.super_let_bind(name, opt_ty, body);
        }
    }
}
