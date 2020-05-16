use std::fmt;

use crate::ast::*;

use eval::*;

use Term::*;

mod ctx;
mod eval;

pub fn evaluate(term: Term) -> Term {
    term.evaluate()
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Term {
    Var(usize),
    Lit(Literal),
    Abs(Box<Term>),
    UnaryOp(UnOp, Box<Term>),
    BinaryOp(BinOp, Box<Term>, Box<Term>),
    App(Box<Term>, Box<Term>),
    Cond(Box<Term>, Box<Term>, Box<Term>),
    Fix(Box<Term>),
    Hole,
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

    fn step_in_place(&mut self) -> bool {
        let term = std::mem::replace(self, Hole);
        let (cont, term) = term.step();
        *self = term;
        cont
    }

    fn step(mut self) -> (bool, Term) {
        match self {
            // Dispatch step for binary operations
            BinaryOp(op, t1, t2) => step_bin_op(op, t1, t2),
            // Dispatch step for unary operations
            UnaryOp(op, t1) => step_un_op(op, t1),
            // Dispatch step for beta reduction
            App(box Abs(body), arg) => step_beta_reduction(body, arg),
            // Application with unevaluated first term (t1 t2)
            // Evaluate t1.
            App(ref mut t1, _) => (t1.step_in_place(), self),
            // Dispatch step for conditionals
            Cond(t1, t2, t3) => step_conditional(t1, t2, t3),
            // Fixed-point operation (fix t1)
            // If t1 is an abstraction (\. t2), replace the argument of t1 by (fix t1) inside t2
            // and evaluate to t2.
            Fix(box Abs(box ref t2)) => {
                let mut t2 = t2.clone();
                t2.replace(0, &mut self);
                (true, t2)
            }
            // If t1 is not an abstraction, evaluate it.
            Fix(ref mut t1) => (t1.step_in_place(), self),
            // Any other term stops the evaluation.
            Var(_) | Lit(_) | Abs(_) | Hole => (false, self),
        }
    }

    fn evaluate(self) -> Term {
        let mut term = self;
        while {
            let (eval, new_term) = term.step();
            term = new_term;
            eval
        } {}
        term
    }
}
