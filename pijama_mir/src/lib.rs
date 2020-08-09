mod lower;

use pijama_ctx::{TermId, LocalId, Context};
use pijama_common::{BinOp, Literal, UnOp};

#[derive(Debug)]
pub enum LetKind {
    NonRec,
    Rec,
}

#[derive(Debug)]
pub enum PrimFn {
    Print,
    BinOp(BinOp),
    UnOp(UnOp),
}

#[derive(Debug)]
pub struct Term {
    id: TermId,
    kind: TermKind,
}

#[derive(Debug)]
pub enum TermKind {
    Lit(Literal),
    Var(LocalId),
    Abs(Vec<LocalId>, Box<Term>),
    App(Box<Term>, Vec<Term>),
    PrimApp(PrimFn, Vec<Term>),
    Cond(Box<Term>, Box<Term>, Box<Term>),
    Let(LetKind, LocalId, Box<Term>, Box<Term>),
}

impl<'ast> Term {
    pub fn from_hir(term: &pijama_hir::Term, ctx: &mut Context) -> Term {
        lower::lower_term(term, ctx)
    }
}
