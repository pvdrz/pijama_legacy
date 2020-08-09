use std::fmt::{self, Debug};

use pijama_common::{BinOp, Literal, Local, Primitive, UnOp};
use pijama_ctx::{Context, LocalId, TermId};

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

    pub fn show(&self, ctx: &Context) {
        println!("{}", TermCtx::new(self, ctx))
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

struct TermCtx<'ast, 'ctx> {
    term: &'ctx Term,
    ctx: &'ctx Context<'ast>,
}

impl<'ast, 'ctx> TermCtx<'ast, 'ctx> {
    fn new(term: &'ctx Term, ctx: &'ctx Context<'ast>) -> Self {
        Self { term, ctx }
    }

    fn get_local(&self, id: LocalId) -> Local<'ast> {
        self.ctx.get_local(id).unwrap()
    }

    fn spawn(&self, term: &'ctx Term) -> Self {
        Self::new(term, self.ctx)
    }
}

impl<'ast, 'ctx> fmt::Display for TermCtx<'ast, 'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.term.kind {
            TermKind::Lit(lit) => write!(f, "{}", lit),
            TermKind::PrimFn(prim) => write!(f, "{}", prim),
            TermKind::Var(local_id) => write!(f, "{}", self.get_local(*local_id)),
            TermKind::Abs(arg_id, body) => write!(
                f,
                "(λ{}. {})",
                self.get_local(*arg_id),
                self.spawn(body.as_ref())
            ),
            TermKind::App(func, arg) => write!(
                f,
                "({} {})",
                self.spawn(func.as_ref()),
                self.spawn(arg.as_ref())
            ),
            TermKind::UnaryOp(op, term) => write!(f, "({}{})", op, self.spawn(term.as_ref())),
            TermKind::BinaryOp(op, t1, t2) => write!(
                f,
                "({} {} {})",
                self.spawn(t1.as_ref()),
                op,
                self.spawn(t2.as_ref())
            ),
            TermKind::Cond(if_term, do_term, else_term) => write!(
                f,
                "(if {} then {} else {})",
                self.spawn(if_term.as_ref()),
                self.spawn(do_term.as_ref()),
                self.spawn(else_term.as_ref())
            ),
            TermKind::Let(kind, lhs_id, rhs, tail) => {
                let kind = match kind {
                    BindKind::NonRec => "",
                    BindKind::Rec => "rec",
                };
                write!(
                    f,
                    "(let{} {} = {} in {})",
                    kind,
                    self.get_local(*lhs_id),
                    self.spawn(rhs.as_ref()),
                    self.spawn(tail.as_ref())
                )
            }
        }
    }
}
