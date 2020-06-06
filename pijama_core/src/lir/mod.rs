use std::fmt;

use pijama_ast::{
    location::Located,
    node::{BinOp, Literal, Primitive, UnOp},
};

use Term::*;

mod lower;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Term {
    Var(usize),
    Lit(i64),
    Abs(Box<Term>),
    UnaryOp(UnOp, Box<Term>),
    BinaryOp(BinOp, Box<Term>, Box<Term>),
    App(Box<Term>, Box<Term>),
    Cond(Box<Term>, Box<Term>, Box<Term>),
    Fix(Box<Term>),
    PrimFn(Primitive),
}

impl Term {
    pub fn as_bool(&self) -> bool {
        match self {
            Lit(0) => false,
            Lit(1) => true,
            _ => panic!("Non-boolean literal {}", self),
        }
    }
}

impl From<Literal> for Term {
    fn from(l: Literal) -> Self {
        match l {
            Literal::Bool(b) => b.into(),
            Literal::Unit => ().into(),
            Literal::Number(n) => n.into(),
        }
    }
}

impl From<i64> for Term {
    fn from(i: i64) -> Self {
        Lit(i)
    }
}

impl From<bool> for Term {
    fn from(b: bool) -> Self {
        Lit(b.into())
    }
}

impl From<()> for Term {
    fn from(_: ()) -> Self {
        Lit(0)
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Var(var) => write!(f, "_{}", var),
            Abs(term) => write!(f, "(λ. {})", term),
            UnaryOp(op, term) => write!(f, "({}{})", op, term),
            BinaryOp(op, t1, t2) => write!(f, "({} {} {})", t1, op, t2),
            App(t1, t2) => write!(f, "({} {})", t1, t2),
            Lit(literal) => write!(f, "{}", literal),
            Cond(t1, t2, t3) => write!(f, "(if {} then {} else {})", t1, t2, t3),
            Fix(t1) => write!(f, "(fix {})", t1),
            PrimFn(prim) => write!(f, "{}", prim),
        }
    }
}

impl Term {
    pub fn from_mir(mir: Located<crate::mir::Term>) -> Self {
        lower::remove_names(mir)
    }

    pub(crate) fn shift(&mut self, up: bool, cutoff: usize) {
        match self {
            Lit(_) | PrimFn(_) => (),
            Var(index) => {
                if *index >= cutoff {
                    if up {
                        *index += 1;
                    } else {
                        *index -= 1;
                    }
                }
            }
            Abs(body) => {
                body.shift(up, cutoff + 1);
            }
            UnaryOp(_, t1) => {
                t1.shift(up, cutoff);
            }
            BinaryOp(_, t1, t2) => {
                t1.shift(up, cutoff);
                t2.shift(up, cutoff);
            }
            App(t1, t2) => {
                t1.shift(up, cutoff);
                t2.shift(up, cutoff);
            }
            Cond(t1, t2, t3) => {
                t1.shift(up, cutoff);
                t2.shift(up, cutoff);
                t3.shift(up, cutoff);
            }
            Fix(t1) => {
                t1.shift(up, cutoff);
            }
        }
    }

    pub(crate) fn replace(&mut self, index: usize, subs: &mut Term) {
        match self {
            Lit(_) | PrimFn(_) => (),
            Var(index2) => {
                if index == *index2 {
                    *self = subs.clone();
                }
            }
            Abs(body) => {
                subs.shift(true, 0);
                body.replace(index + 1, subs);
                subs.shift(false, 0);
            }
            UnaryOp(_, t1) => {
                t1.replace(index, subs);
            }
            BinaryOp(_, t1, t2) => {
                t1.replace(index, subs);
                t2.replace(index, subs);
            }
            App(t1, t2) => {
                t1.replace(index, subs);
                t2.replace(index, subs);
            }
            Cond(t1, t2, t3) => {
                t1.replace(index, subs);
                t2.replace(index, subs);
                t3.replace(index, subs);
            }
            Fix(t1) => {
                t1.replace(index, subs);
            }
        }
    }
}
