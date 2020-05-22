use crate::lir::Term;

use std::io::{stdout, Stdout, Write};

mod eval;

pub struct LangEnv<W: Write> {
    pub stdout: W,
}

impl Default for LangEnv<Stdout> {
    fn default() -> Self {
        LangEnv { stdout: stdout() }
    }
}

pub struct Machine<W: Write> {
    env: LangEnv<W>,
}

impl Default for Machine<Stdout> {
    fn default() -> Self {
        Machine {
            env: LangEnv::default(),
        }
    }
}

impl<W: Write> Machine<W> {
    pub fn with_env(env: LangEnv<W>) -> Self {
        Machine { env }
    }
}

impl<W: Write> Machine<W> {
    pub fn evaluate(&mut self, mut term: Term) -> Term {
        while {
            let (eval, new_term) = self.step(term);
            term = new_term;
            eval
        } {}
        term
    }
}
