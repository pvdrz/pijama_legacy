mod lower;

use pijama_common::{BinOp, Literal, UnOp};
use pijama_ctx::{Context, LocalId, TermId};

#[derive(Debug, Clone)]
pub enum BindKind {
    NonRec,
    Rec,
}

#[derive(Debug, Clone)]
pub enum PrimFn {
    Print,
    BinOp(BinOp),
    UnOp(UnOp),
}

impl PrimFn {
    pub fn arity(&self) -> usize {
        match self {
            Self::Print | Self::UnOp(_) => 1,
            Self::BinOp(_) => 2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Term {
    pub id: TermId,
    pub kind: TermKind,
}

#[derive(Debug, Clone)]
pub enum TermKind {
    Lit(Literal),
    Var(LocalId),
    App(Box<Term>, Vec<Term>),
    PrimApp(PrimFn, Vec<Term>),
    Cond(Box<Term>, Box<Term>, Box<Term>),
    Let(LocalId, Box<Lambda>, Box<Term>),
}

#[derive(Debug, Clone)]
pub struct Lambda(pub TermId, pub Vec<LocalId>, pub Term);

impl Lambda {
    pub fn thunk(term: Term) -> Self {
        Lambda(term.id, vec![], term)
    }
}


impl<'ast> Term {
    pub fn from_hir(term: &pijama_hir::Term, ctx: &mut Context) -> Term {
        lower::lower_term(term, ctx)
    }
}
