mod lower;

use std::fmt;

use pijama_common::{BinOp, Literal, Local, UnOp};
use pijama_ctx::{Context, LocalId, TermId};

#[derive(Debug)]
pub enum BindKind {
    NonRec,
    Rec,
}

#[derive(Debug)]
pub enum PrimFn {
    Print,
    BinOp(BinOp),
    UnOp(UnOp),
}
impl fmt::Display for PrimFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrimFn::Print => write!(f, "print"),
            PrimFn::BinOp(op) => write!(f, "{}", op),
            PrimFn::UnOp(op) => write!(f, "{}", op),
        }
    }
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
    Let(BindKind, LocalId, Box<Term>, Box<Term>),
}

impl<'ast> Term {
    pub fn from_hir(term: &pijama_hir::Term, ctx: &mut Context) -> Term {
        lower::lower_term(term, ctx)
    }

    pub fn show(&self, ctx: &Context) {
        println!("{}", TermCtx::new(self, ctx))
    }
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
            TermKind::Var(local_id) => write!(f, "{}", self.get_local(*local_id)),
            TermKind::Abs(args, body) => {
                write!(f, "(λ")?;
                for arg_id in args {
                    write!(f, " {}", self.get_local(*arg_id))?;
                }
                write!(f, ". {})", self.spawn(body.as_ref()))
            }
            TermKind::App(func, args) => {
                write!(f, "({}", self.spawn(func.as_ref()))?;
                for arg in args {
                    write!(f, " {}", self.spawn(arg))?;
                }
                write!(f, ")")
            }
            TermKind::PrimApp(prim, args) => {
                write!(f, "({}", prim)?;
                for arg in args {
                    write!(f, " {}", self.spawn(arg))?;
                }
                write!(f, ")")
            }
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
