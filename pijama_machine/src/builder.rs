use std::io::{Stdout, Write};

use crate::{
    arithmetic::{Arithmetic, OverflowArithmetic},
    env::Env,
    Machine,
};

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
            _arithmetic: self.arithmetic,
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
