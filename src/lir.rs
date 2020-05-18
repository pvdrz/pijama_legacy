use std::fmt;

use crate::ast::*;
use crate::LangEnv;

use eval::*;
use Term::*;

use lazy_static::lazy_static;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};

mod ctx;
mod eval;

pub fn evaluate(term: Term, env: &mut LangEnv) -> Term {
    term.evaluate(env)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Term {
    Var(usize),
    Lit(Literal),
    Abs(Box<Term>),
    UnaryOp(UnOp, Box<Term>),
    BinaryOp(BinOp, Box<Term>, Box<Term>),
    App(Box<Term>, Box<Term>),
    BuiltInFn(BuiltInFn),
    Cond(Box<Term>, Box<Term>, Box<Term>),
    Fix(Box<Term>),
    Hole,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BuiltInFn {
    Print,
}

impl Display for BuiltInFn {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BuiltInFn::Print => write!(f, "print"),
        }
    }
}

lazy_static! {
    pub static ref BUILT_IN_FNS: HashMap<&'static str, BuiltInFn> = {
        let mut m = HashMap::new();
        m.insert("print", BuiltInFn::Print);
        m
    };
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Hole => write!(f, "hole"),
            Var(var) => write!(f, "_{}", var),
            Abs(term) => write!(f, "(λ. {})", term),
            UnaryOp(op, term) => write!(f, "({}{})", op, term),
            BinaryOp(op, t1, t2) => write!(f, "({} {} {})", t1, op, t2),
            App(t1, t2) => write!(f, "({} {})", t1, t2),
            BuiltInFn(t) => write!(f, "(builtin {})", t),
            Lit(literal) => write!(f, "{}", literal),
            Cond(t1, t2, t3) => write!(f, "(if {} then {} else {})", t1, t2, t3),
            Fix(t1) => write!(f, "(fix {})", t1),
        }
    }
}

impl Term {
    pub fn from_mir(mir: crate::mir::Term) -> Self {
        ctx::remove_names(mir)
    }

    fn shift(&mut self, up: bool, cutoff: usize) {
        match self {
            Lit(_) | Hole => (),
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
            BuiltInFn(_) => {}
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

    fn replace(&mut self, index: usize, subs: &mut Term) {
        match self {
            Lit(_) | Hole => (),
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
            BuiltInFn(_) => {}
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

    fn step_in_place(&mut self, env: &mut LangEnv) -> bool {
        let term = std::mem::replace(self, Hole);
        let (cont, term) = term.step(env);
        *self = term;
        cont
    }

    fn step(mut self, env: &mut LangEnv) -> (bool, Term) {
        match self {
            // Dispatch step for binary operations
            BinaryOp(op, t1, t2) => step_bin_op(op, t1, t2, env),
            // Dispatch step for unary operations
            UnaryOp(op, t1) => step_un_op(op, t1, env),
            // Dispatch step for beta reduction
            App(box Abs(body), arg) => step_beta_reduction(body, arg),
            // Builtin function special handling necessary
            App(box BuiltInFn(BuiltInFn::Print), ref t1) => {
                writeln!(env.stdout, "{}", t1).expect("Print failed");
                (true, Term::Lit(Literal::Unit))
            }
            // Application with unevaluated first term (t1 t2)
            // Evaluate t1.
            App(ref mut t1, _) => (t1.step_in_place(env), self),
            // Dispatch step for conditionals
            Cond(t1, t2, t3) => step_conditional(t1, t2, t3, env),
            // Dispatch step for fixed point operation
            Fix(t1) => step_fix(t1, env),
            // Any other term stops the evaluation.
            Var(_) | Lit(_) | Abs(_) | BuiltInFn(_) | Hole => (false, self),
        }
    }

    fn evaluate(self, env: &mut LangEnv) -> Term {
        let mut term = self;
        while {
            let (eval, new_term) = term.step(env);
            term = new_term;
            eval
        } {}
        term
    }
}
