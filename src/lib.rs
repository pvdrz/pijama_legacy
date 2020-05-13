#![feature(box_patterns)]

use thiserror::Error;

pub mod ast;
pub mod lir;
pub mod mir;
pub mod parser;
pub mod ty;

pub type LangResult<T> = Result<T, LangError>;

#[derive(Error, Debug)]
pub enum LangError {
    #[error("{0}")]
    Ty(#[from] ty::TyError),
    #[error("{0}")]
    Eval(#[from] lir::EvalError),
    #[error("{0}")]
    Parse(String),
}

pub fn run(input: &str) -> LangResult<lir::Term> {
    let ast = parser::parse(input)?;
    let mir = mir::Term::from_ast(ast);
    let _ty = ty::ty_check(&mir)?;
    let lir = lir::Term::from_mir(mir);
    let res = lir::evaluate(lir)?;
    Ok(res)
}
