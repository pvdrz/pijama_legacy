#![feature(box_patterns)]

mod ast;
mod ctx;
mod lir;
mod mir;
mod parser;
mod ty;
mod ty_check;

use std::fs::read_to_string;
use std::env::args;

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
    ty_check::ty_check(&mir)?;
    let mut lir = ctx::remove_names(mir);
    lir.evaluate();
    println!("{}", lir);
    Ok(())
}

fn main() {
    let mut args = args();
    args.next().unwrap();
    let path = args.next().expect("no path to source code");
    let input = read_to_string(path).unwrap();
    match run(&input) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}
