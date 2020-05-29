use crate::{
    lir::Term,
    machine::arithmetic::{Arithmetic, OverflowArithmetic},
};

use std::io::{stdout, Stdout, Write};

pub mod arithmetic;
mod eval;

pub struct LangEnv<W: Write> {
    pub stdout: W,
}

impl Default for LangEnv<Stdout> {
    fn default() -> Self {
        LangEnv { stdout: stdout() }
    }
}

pub struct Machine<W: Write, A: Arithmetic> {
    env: LangEnv<W>,
    arithmetic: A,
}

impl Default for Machine<Stdout, OverflowArithmetic> {
    fn default() -> Self {
        Machine {
            env: LangEnv::default(),
            arithmetic: OverflowArithmetic,
        }
    }
}

impl<W: Write> Machine<W, OverflowArithmetic> {
    pub fn with_env(env: LangEnv<W>) -> Self {
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
