use std::{include_str, time::Duration};

use pijama_ast::Literal;

use pijama::{
    lir::Term,
    machine::{env::Env, MachineBuilder},
    run, run_with_machine, LangError, LangResult,
};

use crate::panic_after;

#[test]
fn arithmetic() -> LangResult<'static, ()> {
    let input = include_str!("arithmetic.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(121), term);
    Ok(())
}

#[test]
fn logic() -> LangResult<'static, ()> {
    let input = include_str!("logic.pj");
    let term = run(input)?;
    assert_eq!(term, false.into());
    Ok(())
}

#[test]
fn factorial() -> LangResult<'static, ()> {
    let input = include_str!("factorial.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(3_628_800), term);
    Ok(())
}

#[test]
fn factorial_tail() -> LangResult<'static, ()> {
    let input = include_str!("factorial_tail.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(3_628_800), term);
    Ok(())
}

#[test]
fn fancy_max() -> LangResult<'static, ()> {
    let input = include_str!("fancy_max.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(10), term);
    Ok(())
}

#[test]
fn fibonacci() -> LangResult<'static, ()> {
    let input = include_str!("fibonacci.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(21), term);
    Ok(())
}

#[test]
fn fibonacci_tail() -> LangResult<'static, ()> {
    let input = include_str!("fibonacci_tail.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(21), term);
    Ok(())
}

#[test]
fn gcd() -> LangResult<'static, ()> {
    let input = include_str!("gcd.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(1), term);
    Ok(())
}

#[test]
fn ackermann() -> LangResult<'static, ()> {
    let input = include_str!("ackermann.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(5), term);
    Ok(())
}

#[test]
fn calling() -> LangResult<'static, ()> {
    let input = include_str!("calling.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(1), term);
    Ok(())
}

#[test]
fn complex_calling() -> LangResult<'static, ()> {
    let input = include_str!("complex_calling.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(1), term);
    Ok(())
}

#[test]
fn step() -> LangResult<'static, ()> {
    let input = include_str!("step.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(1), term);
    Ok(())
}

#[test]
fn bit_and() -> LangResult<'static, ()> {
    let input = include_str!("bit_and.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(64), term);
    Ok(())
}

#[test]
fn bit_or() -> LangResult<'static, ()> {
    let input = include_str!("bit_or.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(192), term);
    Ok(())
}

#[test]
fn bit_xor() -> LangResult<'static, ()> {
    let input = include_str!("bit_xor.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(128), term);
    Ok(())
}

#[test]
fn bit_shift_l() -> LangResult<'static, ()> {
    let input = include_str!("bit_shift_l.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(128), term);
    Ok(())
}

#[test]
fn bit_shift_r() -> LangResult<'static, ()> {
    let input = include_str!("bit_shift_r.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(32), term);
    Ok(())
}

#[test]
fn or_short_circuit() -> LangResult<'static, ()> {
    panic_after(Duration::from_secs(1), || {
        let input = include_str!("or_short_circuit.pj");
        let term = run(input)?;
        assert_eq!(term, true.into());
        Ok(())
    })
}

#[test]
fn and_short_circuit() -> LangResult<'static, ()> {
    panic_after(Duration::from_secs(1), || {
        let input = include_str!("and_short_circuit.pj");
        let term = run(input)?;
        assert_eq!(term, false.into());
        Ok(())
    })
}

#[test]
fn print_simple() -> LangResult<'static, ()> {
    let input = include_str!("print_simple.pj");
    let mut output = Vec::new();
    let term = run_with_machine(
        input,
        MachineBuilder::default()
            .with_env(Env::new(&mut output))
            .build(),
    )?;
    let output = String::from_utf8_lossy(&output);
    assert_eq!(output, "10\n");
    assert_eq!(term, Literal::Unit.into());
    Ok(())
}

#[test]
fn print_simple_fn() -> LangResult<'static, ()> {
    let input = include_str!("print_simple_fn.pj");
    let mut output = Vec::new();
    let term = run_with_machine(
        input,
        MachineBuilder::default()
            .with_env(Env::new(&mut output))
            .build(),
    )?;
    let output = String::from_utf8_lossy(&output);
    assert_eq!(output, "((λ. _0) 10)\n");
    assert_eq!(term, Literal::Unit.into());
    Ok(())
}

#[test]
fn print_complex_fn() -> LangResult<'static, ()> {
    let input = include_str!("print_complex_fn.pj");
    let mut output = Vec::new();
    let term = run_with_machine(
        input,
        MachineBuilder::default()
            .with_env(Env::new(&mut output))
            .build(),
    )?;
    let output = String::from_utf8_lossy(&output);
    assert_eq!(output, "((λ. (if (_0 > 0) then 1 else 0)) 10)\n");
    assert_eq!(term, Literal::Unit.into());
    Ok(())
}

#[test]
fn print_print() -> LangResult<'static, ()> {
    let input = include_str!("print_print.pj");
    let mut output = Vec::new();
    let term = run_with_machine(
        input,
        MachineBuilder::default()
            .with_env(Env::new(&mut output))
            .build(),
    )?;
    let output = String::from_utf8_lossy(&output);
    assert_eq!(output, "(print 10)\n");
    assert_eq!(term, Literal::Unit.into());
    Ok(())
}

#[test]
fn print_redefine() {
    let input = include_str!("print_redefine.pj");
    let err = run(input).unwrap_err();
    assert!(matches!(err, LangError::Parse(_)))
}

#[test]
fn number_bases_cmp() -> LangResult<'static, ()> {
    let input = include_str!("number_bases_cmp.pj");
    let term = run(input)?;
    assert_eq!(term, true.into());
    Ok(())
}

#[test]
fn number_bases_arithmetic() -> LangResult<'static, ()> {
    let input = include_str!("number_bases_arithmetic.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(567_883 * 4), term);
    Ok(())
}
