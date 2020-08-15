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

impl Term {
    fn into_rvalue(self) -> RValue {
        RValue {
            id: self.id,
            kind: RValueKind::Term(self.kind),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TermKind {
    Lit(Literal),
    Var(LocalId),
    App(Box<Term>, Vec<Term>),
    PrimApp(PrimFn, Vec<Term>),
    Cond(Box<Term>, Box<Term>, Box<Term>),
    Let(BindKind, LocalId, Box<RValue>, Box<Term>),
}

#[derive(Debug, Clone)]
pub struct RValue {
    pub id: TermId,
    pub kind: RValueKind,
}

#[derive(Debug, Clone)]
pub enum RValueKind {
    Term(TermKind),
    Abs(Vec<LocalId>, Term),
}

impl<'ast> Term {
    pub fn from_hir(term: &pijama_hir::Term, ctx: &mut Context) -> Term {
        lower::lower_term(term, ctx)
    }
}
