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

pub struct MachineBuilder<W: Write, A: Arithmetic> {
    env: Env<W>,
    arithmetic: A,
}

impl Default for MachineBuilder<Stdout, OverflowArithmetic> {
    fn default() -> Self {
        MachineBuilder {
            env: Env::default(),
            arithmetic: OverflowArithmetic,
        }
    }
}

impl<W: Write, A: Arithmetic> MachineBuilder<W, A> {
    pub fn build(self) -> Machine<W, A> {
        Machine {
            env: self.env,
            arithmetic: self.arithmetic,
        }
    }

    pub fn with_env<W2: Write>(self, env: Env<W2>) -> MachineBuilder<W2, A> {
        MachineBuilder {
            env,
            arithmetic: self.arithmetic,
        }
    }

    pub fn with_arithmetic<A2: Arithmetic>(self, arithmetic: A2) -> MachineBuilder<W, A2> {
        MachineBuilder {
            env: self.env,
            arithmetic,
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
