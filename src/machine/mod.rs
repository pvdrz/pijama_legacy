use std::io::{Stdout, Write};

use crate::{
    lir::Term,
    machine::{
        arithmetic::{Arithmetic, OverflowArithmetic},
        env::Env,
    },
};

pub mod arithmetic;
pub mod env;
mod eval;

pub struct Machine<W: Write, A: Arithmetic> {
    env: Env<W>,
    arithmetic: A,
}

impl Default for Machine<Stdout, OverflowArithmetic> {
    fn default() -> Self {
        Machine {
            env: Env::default(),
            arithmetic: OverflowArithmetic,
        }
    }
}

impl<W: Write> Machine<W, OverflowArithmetic> {
    pub fn with_env(env: Env<W>) -> Self {
        Machine {
            env,
            arithmetic: OverflowArithmetic,
        }
    }
}

impl<W: Write, A: Arithmetic> Machine<W, A> {
    pub fn evaluate(&mut self, mut term: Term) -> Term {
        while {
            let (eval, new_term) = self.step(term);
            term = new_term;
            eval
        } {}
        term
    }
}
