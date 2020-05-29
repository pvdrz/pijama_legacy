use std::io::{stdout, Stdout, Write};

pub struct Env<W: Write> {
    pub stdout: W,
}

impl Default for Env<Stdout> {
    fn default() -> Self {
        Env { stdout: stdout() }
    }
}
