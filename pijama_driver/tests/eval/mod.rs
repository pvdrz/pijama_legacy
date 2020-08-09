use std::{include_str, time::Duration};

use pijama_driver::{LangErrorKind, LangResult};

use crate::{panic_after, run};

#[test]
fn arithmetic() -> LangResult<()> {
    let input = include_str!("arithmetic.pj");
    let output = run(input)?;
    assert_eq!("121\n", output);
    Ok(())
}

#[test]
fn logic() -> LangResult<()> {
    let input = include_str!("logic.pj");
    let output = run(input)?;
    assert_eq!("false\n", output);
    Ok(())
}

#[test]
fn factorial() -> LangResult<()> {
    let input = include_str!("factorial.pj");
    let output = run(input)?;
    assert_eq!("3628800\n", output);
    Ok(())
}

#[test]
fn factorial_tail() -> LangResult<()> {
    let input = include_str!("factorial_tail.pj");
    let output = run(input)?;
    assert_eq!("3628800\n", output);
    Ok(())
}

#[test]
fn fancy_max() -> LangResult<()> {
    let input = include_str!("fancy_max.pj");
    let output = run(input)?;
    assert_eq!("10\n", output);
    Ok(())
}

#[test]
fn fibonacci() -> LangResult<()> {
    let input = include_str!("fibonacci.pj");
    let output = run(input)?;
    assert_eq!("21\n", output);
    Ok(())
}

#[test]
fn fibonacci_tail() -> LangResult<()> {
    let input = include_str!("fibonacci_tail.pj");
    let output = run(input)?;
    assert_eq!("21\n", output);
    Ok(())
}

#[test]
fn gcd() -> LangResult<()> {
    let input = include_str!("gcd.pj");
    let output = run(input)?;
    assert_eq!("1\n", output);
    Ok(())
}

#[test]
fn ackermann() -> LangResult<()> {
    let input = include_str!("ackermann.pj");
    let output = run(input)?;
    assert_eq!("5\n", output);
    Ok(())
}

#[test]
fn calling() -> LangResult<()> {
    let input = include_str!("calling.pj");
    let output = run(input)?;
    assert_eq!("1\n", output);
    Ok(())
}

#[test]
fn complex_calling() -> LangResult<()> {
    let input = include_str!("complex_calling.pj");
    let output = run(input)?;
    assert_eq!("1\n", output);
    Ok(())
}

#[test]
fn step() -> LangResult<()> {
    let input = include_str!("step.pj");
    let output = run(input)?;
    assert_eq!("1\n", output);
    Ok(())
}

#[test]
fn bit_and() -> LangResult<()> {
    let input = include_str!("bit_and.pj");
    let output = run(input)?;
    assert_eq!("64\n", output);
    Ok(())
}

#[test]
fn bit_or() -> LangResult<()> {
    let input = include_str!("bit_or.pj");
    let output = run(input)?;
    assert_eq!("192\n", output);
    Ok(())
}

#[test]
fn bit_xor() -> LangResult<()> {
    let input = include_str!("bit_xor.pj");
    let output = run(input)?;
    assert_eq!("128\n", output);
    Ok(())
}

#[test]
fn bit_shift_l() -> LangResult<()> {
    let input = include_str!("bit_shift_l.pj");
    let output = run(input)?;
    assert_eq!("128\n", output);
    Ok(())
}

#[test]
fn bit_shift_r() -> LangResult<()> {
    let input = include_str!("bit_shift_r.pj");
    let output = run(input)?;
    assert_eq!("32\n", output);
    Ok(())
}

#[test]
fn or_short_circuit() -> LangResult<()> {
    panic_after(Duration::from_secs(1), || {
        let input = include_str!("or_short_circuit.pj");
        let output = run(input)?;
        assert_eq!("true\n", output);
        Ok(())
    })
}

#[test]
fn and_short_circuit() -> LangResult<()> {
    panic_after(Duration::from_secs(1), || {
        let input = include_str!("and_short_circuit.pj");
        let output = run(input)?;
        assert_eq!("false\n", output);
        Ok(())
    })
}

#[test]
fn print_simple() -> LangResult<()> {
    let input = include_str!("print_simple.pj");
    let output = run(input)?;
    assert_eq!("10\n", output);
    Ok(())
}

#[test]
fn print_simple_fn() -> LangResult<()> {
    let input = include_str!("print_simple_fn.pj");
    let output = run(input)?;
    assert_eq!("<function>\n", output);
    Ok(())
}

#[test]
fn print_complex_fn() -> LangResult<()> {
    let input = include_str!("print_complex_fn.pj");
    let output = run(input)?;
    assert_eq!("1\n", output);
    Ok(())
}

#[test]
fn print_print() -> LangResult<()> {
    let input = include_str!("print_print.pj");
    let output = run(input)?;
    assert_eq!("10\nunit\n", output);
    Ok(())
}

#[test]
fn print_redefine() {
    let input = include_str!("print_redefine.pj");
    let err = run(input).unwrap_err();
    assert!(matches!(err.kind(), LangErrorKind::Parse(_)))
}

#[test]
fn number_bases_cmp() -> LangResult<()> {
    let input = include_str!("number_bases_cmp.pj");
    let output = run(input)?;
    assert_eq!("true\n", output);
    Ok(())
}

#[test]
fn number_bases_arithmetic() -> LangResult<()> {
    let input = include_str!("number_bases_arithmetic.pj");
    let output = run(input)?;
    assert_eq!("2271532\n", output);
    Ok(())
}

#[test]
#[should_panic]
fn add_overflow_panics() {
    let input = include_str!("add_overflow_panics.pj");
    run(input).ok();
}

#[test]
#[should_panic]
fn neg_overflow_panics() {
    let input = include_str!("neg_overflow_panics.pj");
    run(input).ok();
}

#[test]
fn short_circuit() -> LangResult<()> {
    let input = include_str!("short_circuit.pj");
    let output = run(input)?;
    assert_eq!("true\n", output);
    Ok(())
}

#[test]
fn cond_chain() -> LangResult<()> {
    let input = include_str!("cond_chain.pj");
    let output = run(input)?;
    assert_eq!("true\n", output);
    Ok(())
}

#[test]
fn adler32() -> LangResult<()> {
    let input = include_str!("adler32.pj");
    let output = run(input)?;
    assert_eq!("300286872\n", output);
    Ok(())
}
