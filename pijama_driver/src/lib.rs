use thiserror::Error;

use std::io::Write;

use pijama_parser::{parse, ParsingError};

use pijama_hir::LowerError;

use pijama_ty::ty_gen;

use pijama_tycheck::{ty_check, TyError};

use pijama_lir::Term as LirTerm;

use pijama_machine::{
    arithmetic::{Arithmetic, CheckedArithmetic, OverflowArithmetic},
    Machine, MachineBuilder,
};

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
    let (hir, ctx) = pijama_hir::lower_block(ast, ty_gen())?;
    let _ty = ty_check(&hir, ctx)?;
    let lir = LirTerm::from_hir(hir);
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
