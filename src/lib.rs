#![feature(box_patterns)]

use thiserror::Error;

use machine::Machine;
use parser::ParseError;
use ty::TyError;

pub mod ast;
pub mod lir;
pub mod machine;
pub mod mir;
pub mod parser;
pub mod ty;

pub type LangResult<'a, T> = Result<T, LangError<'a>>;

#[derive(Error, Debug)]
pub enum LangError<'a> {
    #[error("Type error: {0}")]
    Ty(#[from] TyError),
    #[error("Parse error: {0}")]
    Parse(ParseError<'a>),
}

impl<'a> From<ParseError<'a>> for LangError<'a> {
    fn from(err: ParseError<'a>) -> Self {
        LangError::Parse(err)
    }
}

pub fn run(input: &str) -> LangResult<lir::Term> {
    let ast = parser::parse(input)?;
    let mir = mir::Term::from_ast(ast)?;
    let _ty = ty::ty_check(&mir)?;
    let lir = lir::Term::from_mir(mir);
    let res = Machine::default().evaluate(lir);
    Ok(res)
}
