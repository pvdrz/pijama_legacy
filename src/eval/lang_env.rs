use std::io::{stdout, Stdout, Write};

use crate::eval::{CheckedMachine, Machine, OverflowMachine};

pub struct LangEnv<W: Write> {
    stdout: W,
}

impl<W: Write> LangEnv<W> {
    pub fn new(stdout: W) -> Self {
        LangEnv { stdout }
    }

    pub fn stdout(&mut self) -> &mut W {
        &mut self.stdout
    }
}

impl Default for LangEnv<Stdout> {
    fn default() -> Self {
        LangEnv { stdout: stdout() }
    }
}

impl Default for OverflowMachine<Stdout> {
    fn default() -> Self {
        Self::with_env(LangEnv::default())
    }
}

impl Default for CheckedMachine<Stdout> {
    fn default() -> Self {
        Self::with_env(LangEnv::default())
    }
}
