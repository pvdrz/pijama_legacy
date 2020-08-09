use std::fmt::Debug;

use pijama_common::{BinOp, Literal, Primitive, UnOp};

use pijama_ctx::{LocalId, TermId};

pub use lower::{lower_ast, LowerError, LowerErrorKind, LowerResult};

mod lower;

#[derive(Debug, Copy, Clone)]
pub enum BindKind {
    NonRec,
    Rec,
}

#[derive(Debug)]
pub struct Term {
    pub id: TermId,
    pub kind: TermKind,
}

impl Term {
    pub(crate) fn new(id: TermId, kind: TermKind) -> Self {
        Self { id, kind }
    }
}

#[derive(Debug)]
pub enum TermKind {
    Lit(Literal),
    PrimFn(Primitive),
    Var(LocalId),
    Abs(LocalId, Box<Term>),
    App(Box<Term>, Box<Term>),
    UnaryOp(UnOp, Box<Term>),
    BinaryOp(BinOp, Box<Term>, Box<Term>),
    Cond(Box<Term>, Box<Term>, Box<Term>),
    Let(BindKind, LocalId, Box<Term>, Box<Term>),
}
