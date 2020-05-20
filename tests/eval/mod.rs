use std::include_str;

use crate::panic_after;
use pijama::ast::Literal;
use pijama::lir::Term;
use pijama::machine::{LangEnv, Machine};
use pijama::{run, run_with_machine, LangError, LangResult};
use std::time::Duration;

#[test]
fn arithmetic() -> LangResult<()> {
    let input = include_str!("arithmetic.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(Literal::Number(121)), term);
    Ok(())
}

#[test]
fn logic() -> LangResult<()> {
    let input = include_str!("logic.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(Literal::Bool(false)), term);
    Ok(())
}

#[test]
fn factorial() -> LangResult<()> {
    let input = include_str!("factorial.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(Literal::Number(3628800)), term);
    Ok(())
}

#[test]
fn factorial_tail() -> LangResult<()> {
    let input = include_str!("factorial_tail.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(Literal::Number(3628800)), term);
    Ok(())
}

#[test]
fn fancy_max() -> LangResult<()> {
    let input = include_str!("fancy_max.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(Literal::Number(10)), term);
    Ok(())
}

#[test]
fn fibonacci() -> LangResult<()> {
    let input = include_str!("fibonacci.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(Literal::Number(21)), term);
    Ok(())
}

#[test]
fn fibonacci_tail() -> LangResult<()> {
    let input = include_str!("fibonacci_tail.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(Literal::Number(21)), term);
    Ok(())
}

#[test]
fn gcd() -> LangResult<()> {
    let input = include_str!("gcd.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(Literal::Number(1)), term);
    Ok(())
}

#[test]
fn ackermann() -> LangResult<()> {
    let input = include_str!("ackermann.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(Literal::Number(5)), term);
    Ok(())
}

#[test]
fn calling() -> LangResult<()> {
    let input = include_str!("calling.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(Literal::Number(1)), term);
    Ok(())
}

#[test]
fn complex_calling() -> LangResult<()> {
    let input = include_str!("complex_calling.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(Literal::Number(1)), term);
    Ok(())
}

#[test]
fn step() -> LangResult<()> {
    let input = include_str!("step.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(Literal::Number(1)), term);
    Ok(())
}

#[test]
fn bit_and() -> LangResult<()> {
    let input = include_str!("bit_and.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(Literal::Number(64)), term);
    Ok(())
}

#[test]
fn bit_or() -> LangResult<()> {
    let input = include_str!("bit_or.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(Literal::Number(192)), term);
    Ok(())
}

#[test]
fn bit_xor() -> LangResult<()> {
    let input = include_str!("bit_xor.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(Literal::Number(128)), term);
    Ok(())
}

#[test]
fn bit_shift_l() -> LangResult<()> {
    let input = include_str!("bit_shift_l.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(Literal::Number(128)), term);
    Ok(())
}

#[test]
fn bit_shift_r() -> LangResult<()> {
    let input = include_str!("bit_shift_r.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(Literal::Number(32)), term);
    Ok(())
}

#[test]
fn or_short_circuit() -> LangResult<()> {
    panic_after(Duration::from_secs(1), || {
        let input = include_str!("or_short_circuit.pj");
        let term = run(input)?;
        assert_eq!(Term::Lit(Literal::Bool(true)), term);
        Ok(())
    })
}

#[test]
fn and_short_circuit() -> LangResult<()> {
    panic_after(Duration::from_secs(1), || {
        let input = include_str!("and_short_circuit.pj");
        let term = run(input)?;
        assert_eq!(Term::Lit(Literal::Bool(false)), term);
        Ok(())
    })
}

#[test]
fn print_simple() -> LangResult<()> {
    let input = include_str!("print_simple.pj");
    let mut output = Vec::new();
    let term = run_with_machine(
        input,
        Machine::with_env(LangEnv {
            stdout: &mut output,
        }),
    )?;
    let output = String::from_utf8_lossy(&output);
    assert_eq!(output, "10\n");
    assert_eq!(Term::Lit(Literal::Unit), term);
    Ok(())
}

#[test]
fn print_simple_fn() -> LangResult<()> {
    let input = include_str!("print_simple_fn.pj");
    let mut output = Vec::new();
    let term = run_with_machine(
        input,
        Machine::with_env(LangEnv {
            stdout: &mut output,
        }),
    )?;
    let output = String::from_utf8_lossy(&output);
    assert_eq!(output, "((λ. _0) 10)\n");
    assert_eq!(Term::Lit(Literal::Unit), term);
    Ok(())
}

#[test]
fn print_complex() -> LangResult<()> {
    let input = include_str!("print_complex.pj");
    let mut output = Vec::new();
    let term = run_with_machine(
        input,
        Machine::with_env(LangEnv {
            stdout: &mut output,
        }),
    )?;
    let output = String::from_utf8_lossy(&output);
    assert_eq!(output, "((λ. (if (_0 > 0) then 1 else 0)) 10)\n");
    assert_eq!(Term::Lit(Literal::Unit), term);
    Ok(())
}

#[test]
fn print_print() -> LangResult<()> {
    let input = include_str!("print_print.pj");
    let mut output = Vec::new();
    let term = run_with_machine(
        input,
        Machine::with_env(LangEnv {
            stdout: &mut output,
        }),
    )?;
    let output = String::from_utf8_lossy(&output);
    assert_eq!(output, "((builtin print) 10)\n");
    assert_eq!(Term::Lit(Literal::Unit), term);
    Ok(())
}

#[test]
fn print_redefine() {
    let input = include_str!("print_redefine.pj");
    let err = run(input).unwrap_err();
    assert!(matches!(err, LangError::Parse(_)))
}
