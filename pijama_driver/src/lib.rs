use thiserror::Error;

use std::io::Write;

use pijama_core::{
    lir::Term as LirTerm,
    machine::{
        arithmetic::{Arithmetic, CheckedArithmetic, OverflowArithmetic},
        Machine, MachineBuilder,
    },
    mir::{LowerError, Term as MirTerm},
    parser::{parse, ParsingError},
    ty::{ty_check, TyError},
};

pub type LangResult<'a, T> = Result<T, LangError<'a>>;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum LangError<'a> {
    #[error("{0}")]
    Ty(#[from] TyError),
    #[error("{0}")]
    Parse(ParsingError<'a>),
    #[error("{0}")]
    Lower(#[from] LowerError),
}

impl<'a> From<ParsingError<'a>> for LangError<'a> {
    fn from(err: ParsingError<'a>) -> Self {
        LangError::Parse(err)
    }
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
