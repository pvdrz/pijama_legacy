#![feature(box_patterns)]

use std::io::Write;

use crate::machine::Machine;
use thiserror::Error;

pub mod ast;
pub mod lir;
pub mod machine;
pub mod mir;
pub mod parser;
pub mod ty;

pub type LangResult<T> = Result<T, LangError>;

#[derive(Error, Debug)]
pub enum LangError {
    #[error("{0}")]
    Ty(#[from] ty::TyError),
    #[error("{0}")]
    Parse(String),
}

pub fn run(input: &str) -> LangResult<lir::Term> {
    run_with_machine(input, Machine::default())
}

pub fn run_with_machine<W: Write>(input: &str, mut machine: Machine<W>) -> LangResult<lir::Term> {
    let ast = parser::parse(input)?;
    let mir = mir::Term::from_ast(ast)?;
    let _ty = ty::ty_check(&mir)?;
    let lir = lir::Term::from_mir(mir);
    let res = machine.evaluate(lir);
    Ok(res)
}
