mod ast;
mod ctx;
mod lir;
mod mir;
mod parser;
mod ty;
mod ty_check;

use thiserror::Error;

use parser::parse;

pub type LangResult<T> = Result<T, LangError>;

#[derive(Error, Debug)]
pub enum LangError {
    #[error("{0}")]
    Ty(#[from] ty_check::TyError),
}

fn run(input: &str) -> Result<(), LangError> {
    let ast = parse(input).unwrap();
    let mir = mir::lower(ast);
    println!("{}", mir);
    let ty = ty_check::ty_check(&mir)?;
    println!("{}", ty);
    let mut lir = ctx::remove_names(mir);
    println!("{}", lir);
    lir.evaluate();
    println!("{}", lir);
    Ok(())
}

fn main() {
    let input = include_str!("source.pj");
    match run(input) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}
