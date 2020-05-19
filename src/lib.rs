#![feature(box_patterns)]

use std::io::{self, Write};

use thiserror::Error;
use crate::machine::Machine;

pub mod ast;
pub mod lir;
pub mod mir;
pub mod parser;
pub mod ty;
pub mod machine;

pub type LangResult<T> = Result<T, LangError>;

#[derive(Error, Debug)]
pub enum LangError {
    #[error("{0}")]
    Ty(#[from] ty::TyError),
    #[error("{0}")]
    Parse(String),
}

pub struct LangEnv<'a> {
    pub stdout: &'a mut dyn Write,
}

pub fn run(input: &str) -> LangResult<lir::Term> {
    run_with_env(
        input,
        &mut LangEnv {
            stdout: &mut io::stdout(),
        },
    )
}

pub fn run_with_env(input: &str, env: &mut LangEnv) -> LangResult<lir::Term> {
    let ast = parser::parse(input)?;
    let mir = mir::Term::from_ast(ast)?;
    let _ty = ty::ty_check(&mir)?;
    let lir = lir::Term::from_mir(mir);
    let res = Machine {
        env
    }.evaluate(lir);
    Ok(res)
}
