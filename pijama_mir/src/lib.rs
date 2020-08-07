mod lower;

use pijama_common::{BinOp, Literal, Local, UnOp};

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
pub enum Term<'ast> {
    Lit(Literal),
    Var(Local<'ast>),
    Abs(Vec<Local<'ast>>, Box<Self>),
    App(Box<Self>, Vec<Self>),
    PrimApp(PrimFn, Vec<Self>),
    Cond(Box<Self>, Box<Self>, Box<Self>),
    Let(LetKind, Local<'ast>, Box<Self>, Box<Self>),
}

impl<'ast> Term<'ast> {
    pub fn from_hir(term: &pijama_common::location::Located<pijama_hir::Term<'ast>>) -> Term<'ast> {
        lower::lower_term(term)
    }
}
