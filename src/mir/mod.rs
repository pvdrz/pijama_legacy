use std::fmt::{Display, Formatter, Result};

use crate::{
    ast::{BinOp, Block, Literal, Located, Name, UnOp},
    ty::Binding,
    LangError, LangResult,
};

mod lower;

#[derive(Debug)]
pub enum Term<'a> {
    Var(Name<'a>),
    Abs(Binding<'a>, Box<Located<Term<'a>>>),
    UnaryOp(UnOp, Box<Located<Term<'a>>>),
    BinaryOp(BinOp, Box<Located<Term<'a>>>, Box<Located<Term<'a>>>),
    App(Box<Located<Term<'a>>>, Box<Located<Term<'a>>>),
    Lit(Literal),
    Cond(
        Box<Located<Term<'a>>>,
        Box<Located<Term<'a>>>,
        Box<Located<Term<'a>>>,
    ),
    Let(
        Located<Name<'a>>,
        Box<Located<Term<'a>>>,
        Box<Located<Term<'a>>>,
    ),
    Seq(Box<Located<Term<'a>>>, Box<Located<Term<'a>>>),
    Fix(Box<Located<Term<'a>>>),
}

impl<'a> Display for Term<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Term::Var(var) => write!(f, "{}", var),
            Term::Abs(Binding { name, ty }, term) => write!(f, "(λ{}:{}. {})", name, ty, term),
            Term::UnaryOp(op, term) => write!(f, "({}{})", op, term),
            Term::BinaryOp(op, t1, t2) => write!(f, "({} {} {})", t1, op, t2),
            Term::App(t1, t2) => write!(f, "({} {})", t1, t2),
            Term::Lit(literal) => write!(f, "{}", literal),
            Term::Cond(t1, t2, t3) => write!(f, "(if {} then {} else {})", t1, t2, t3),
            Term::Let(name, t1, t2) => write!(f, "(let {} = {} in {})", name, t1, t2),
            Term::Seq(t1, t2) => write!(f, "{} ; {}", t1, t2),
            Term::Fix(t1) => write!(f, "(fix {})", t1),
        }
    }
}

impl<'a> Term<'a> {
    pub fn from_ast(blk: Located<Block<'a>>) -> LangResult<Located<Self>> {
        lower::lower_blk(blk).map_err(LangError::Ty)
    }
}
