use std::fmt::{Display, Formatter, Result};

use pijama_ast::node::Block;
use pijama_common::{location::Located, BinOp, Literal, Local, Primitive, UnOp};
use pijama_ty::Ty;

pub use lower::{LowerError, LowerResult};

mod lower;

#[derive(Debug)]
pub enum LetKind {
    NonRec(Option<Located<Ty>>),
    Rec(Located<Ty>),
}

#[derive(Debug)]
pub enum Term<'a> {
    Lit(Literal),
    PrimFn(Primitive),
    Var(Local<'a>),
    Abs(Local<'a>, Ty, Box<Located<Self>>),
    App(Box<Located<Self>>, Box<Located<Self>>),
    UnaryOp(UnOp, Box<Located<Self>>),
    BinaryOp(BinOp, Box<Located<Self>>, Box<Located<Self>>),
    Cond(Box<Located<Self>>, Box<Located<Self>>, Box<Located<Self>>),
    Let(
        LetKind,
        Located<Local<'a>>,
        Box<Located<Self>>,
        Box<Located<Self>>,
    ),
}

impl<'a> Display for Term<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Term::Var(var) => write!(f, "{}", var),
            Term::Abs(name, ty, term) => write!(f, "(λ{}:{}. {})", name, ty, term),
            Term::UnaryOp(op, term) => write!(f, "({}{})", op, term),
            Term::BinaryOp(op, t1, t2) => write!(f, "({} {} {})", t1, op, t2),
            Term::App(t1, t2) => write!(f, "({} {})", t1, t2),
            Term::Lit(literal) => write!(f, "{}", literal),
            Term::Cond(t1, t2, t3) => write!(f, "(if {} then {} else {})", t1, t2, t3),
            Term::Let(LetKind::Rec(ty), name, t1, t2) => {
                write!(f, "(let rec {} : {} = {} in {})", name, ty.content, t1, t2)
            }
            Term::Let(LetKind::NonRec(Some(ty)), name, t1, t2) => {
                write!(f, "(let {} : {} = {} in {})", name, ty.content, t1, t2)
            }
            Term::Let(LetKind::NonRec(None), name, t1, t2) => {
                write!(f, "(let {} = {} in {})", name, t1, t2)
            }
            Term::PrimFn(prim) => write!(f, "{}", prim),
        }
    }
}

impl<'a> Term<'a> {
    pub fn from_ast(blk: Block<'a>) -> LowerResult<Located<Self>> {
        lower::lower_block(blk)
    }
}
