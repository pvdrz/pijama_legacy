#![feature(box_patterns)]

mod ast;
mod ctx;
mod lir;
mod mir;
mod parser;
mod ty;
mod ty_check;

use thiserror::Error;

pub type LangResult<T> = Result<T, LangError>;

#[derive(Error, Debug)]
pub enum LangError {
    #[error("{0}")]
    Ty(#[from] ty_check::TyError),
    #[error("{0}")]
    Eval(#[from] lir::EvalError),
    #[error("{0}")]
    Parse(String),
}

pub fn run(input: &str) -> Result<lir::Term, LangError> {
    let ast = parser::parse(input)?;
    let mir = mir::lower(ast);
    ty_check::ty_check(&mir)?;
    let mut lir = ctx::remove_names(mir);
    lir.evaluate()?;
    Ok(lir)
}

