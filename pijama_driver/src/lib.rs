use thiserror::Error;

use std::io::Write;

use pijama_core::{
    lir::Term as LirTerm,
    machine::{
        arithmetic::{Arithmetic, CheckedArithmetic, OverflowArithmetic},
        Machine, MachineBuilder,
    },
    mir::{LowerError, Term as MirTerm},
    ty::{ty_check, TyError},
};
use pijama_parser::{parse, ParsingError};

pub type LangResult<T> = Result<T, LangError>;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum LangError {
    #[error("{0}")]
    Ty(#[from] TyError),
    #[error("{0}")]
    Parse(#[from] ParsingError),
    #[error("{0}")]
    Lower(#[from] LowerError),
}

pub fn run_with_machine<W: Write, A: Arithmetic>(
    input: &str,
    mut machine: Machine<W, A>,
) -> LangResult<()> {
    let ast = parse(input)?;
    let mir = MirTerm::from_ast(ast)?;
    let _ty = ty_check(&mir)?;
    let lir = LirTerm::from_mir(mir);
    let _res = machine.evaluate(lir);
    Ok(())
}

pub fn run(input: &str, overflow_check: bool) -> LangResult<()> {
    if overflow_check {
        let machine = MachineBuilder::default()
            .with_arithmetic(CheckedArithmetic)
            .build();
        run_with_machine(input, machine)
    } else {
        let machine = MachineBuilder::default()
            .with_arithmetic(OverflowArithmetic)
            .build();
        run_with_machine(input, machine)
    }
}
