use std::io::{stdout, Stdout, Write};

pub struct Env<W: Write> {
    stdout: W,
}

impl<W: Write> Env<W> {
    pub fn new(stdout: W) -> Self {
        Env { stdout }
    }

    pub fn stdout(&mut self) -> &mut W {
        &mut self.stdout
    }
}

impl Default for Env<Stdout> {
    fn default() -> Self {
        Env { stdout: stdout() }
    }
}
