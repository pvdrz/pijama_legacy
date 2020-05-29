use crate::lir::Term;

use std::io::{stdout, Stdout, Write};

pub mod eval;

use eval::Machine;

pub struct LangEnv<W: Write> {
    pub stdout: W,
}

impl Default for LangEnv<Stdout> {
    fn default() -> Self {
        LangEnv { stdout: stdout() }
    }
}

pub struct OverflowMachine<W: Write> {
    env: LangEnv<W>,
}

impl Default for OverflowMachine<Stdout> {
    fn default() -> Self {
        OverflowMachine {
            env: LangEnv::default(),
        }
    }
}

impl<W: Write> Machine<W> for OverflowMachine<W> {
    fn lang_env(&mut self) -> &mut LangEnv<W> {
        &mut self.env
    }
}

impl<W: Write> OverflowMachine<W> {
    pub fn with_env(env: LangEnv<W>) -> Self {
        OverflowMachine { env }
    }

    pub fn evaluate(&mut self, term: Term) -> Term {
        Machine::evaluate(self, term)
    }
}
