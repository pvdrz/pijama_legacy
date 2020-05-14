use std::include_str;

use pijama::{LangResult, run};
use pijama::lir::Term;
use pijama::ast::Literal;

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