use std::io::{stdout, Stdout, Write};

pub struct LangEnv<W: Write> {
    pub stdout: W,
}

impl Default for LangEnv<Stdout> {
    fn default() -> Self {
        LangEnv { stdout: stdout() }
    }
}
