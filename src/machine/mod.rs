use std::io::Write;

use crate::{
    lir::Term,
    machine::{arithmetic::Arithmetic, env::Env},
};

pub mod arithmetic;
mod builder;
pub mod env;
mod eval;

pub use builder::MachineBuilder;

pub struct Machine<W: Write, A: Arithmetic> {
    env: Env<W>,
    _arithmetic: A,
}

impl<W: Write, A: Arithmetic> Machine<W, A> {
    pub fn evaluate(&mut self, term: Term) -> Term {
        self.eval(term).1
    }
}
