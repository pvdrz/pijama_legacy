use std::fmt::Debug;

use pijama_common::{BinOp, Literal, Primitive, UnOp};

use pijama_ty::context::{LocalId, TermId};

pub use lower::{lower_ast, LowerError, LowerResult};

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
//
// impl<'a> Display for Term<'a> {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         match self {
//             Term::Var(var) => write!(f, "{}", var),
//             Term::Abs(name, ty, term) => write!(f, "(λ{}:{}. {})", name, ty, term),
//             Term::UnaryOp(op, term) => write!(f, "({}{})", op, term),
//             Term::BinaryOp(op, t1, t2) => write!(f, "({} {} {})", t1, op, t2),
//             Term::App(t1, t2) => write!(f, "({} {})", t1, t2),
//             Term::Lit(literal) => write!(f, "{}", literal),
//             Term::Cond(t1, t2, t3) => write!(f, "(if {} then {} else {})", t1, t2, t3),
//             Term::Let(LetKind::Rec(ty), name, t1, t2) => {
//                 write!(f, "(let rec {} : {} = {} in {})", name, ty.content, t1, t2)
//             }
//             Term::Let(LetKind::NonRec(Some(ty)), name, t1, t2) => {
//                 write!(f, "(let {} : {} = {} in {})", name, ty.content, t1, t2)
//             }
//             Term::Let(LetKind::NonRec(None), name, t1, t2) => {
//                 write!(f, "(let {} = {} in {})", name, t1, t2)
//             }
//             Term::PrimFn(prim) => write!(f, "{}", prim),
//         }
//     }
// }
//
