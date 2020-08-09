use thiserror::Error;

use std::io::Write;

use pijama_common::location::LocatedError;
use pijama_ctx::Context;
use pijama_hir::LowerErrorKind;
use pijama_lir::Term as LirTerm;
use pijama_machine::{
    arithmetic::{Arithmetic, CheckedArithmetic, OverflowArithmetic},
    Machine, MachineBuilder,
};
use pijama_parser::{parse, ParsingErrorKind};
use pijama_tycheck::{ty_check, TyErrorKind};

pub type LangResult<T> = Result<T, LangError>;

pub type LangError = LocatedError<LangErrorKind>;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum LangErrorKind {
    #[error("{0}")]
    Ty(#[from] TyErrorKind),
    #[error("{0}")]
    Parse(#[from] ParsingErrorKind),
    #[error("{0}")]
    Lower(#[from] LowerErrorKind),
}

pub fn run_with_machine<W: Write, A: Arithmetic>(
    input: &str,
    mut machine: Machine<W, A>,
) -> LangResult<()> {
    let ast = parse(input).map_err(LocatedError::kind_into)?;
    let mut ctx = Context::new();
    let hir = pijama_hir::lower_ast(&mut ctx, ast).map_err(LocatedError::kind_into)?;
    let _ty = ty_check(&hir, &mut ctx).map_err(LocatedError::kind_into)?;
    let _mir = pijama_mir::Term::from_hir(&hir, &mut ctx);
    let lir = LirTerm::from_hir(&ctx, hir);
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
