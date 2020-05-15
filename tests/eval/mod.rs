use std::include_str;

use crate::panic_after;
use pijama::ast::Literal;
use pijama::lir::Term;
use pijama::{run, LangResult};
use std::time::Duration;

#[test]
fn arithmetic() -> LangResult<()> {
    let input = include_str!("arithmetic.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(Literal::Number(55)), term);
    Ok(())
}

#[test]
fn fact_rec() -> LangResult<()> {
    let input = include_str!("fact_rec.pj");
    let term = run(input)?;
    assert_eq!(Term::Lit(Literal::Number(2432902008176640000)), term);
    Ok(())
}

#[test]
fn fact_tail() -> LangResult<()> {
    let input = include_str!("fact_tail.pj");
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