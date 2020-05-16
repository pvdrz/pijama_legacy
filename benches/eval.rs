use criterion::{criterion_group, criterion_main, Criterion};

use std::include_str;

use pijama::lir::{evaluate, Term};
use pijama::mir::Term as MirTerm;
use pijama::parser::parse;
use pijama::ty::ty_check;
use pijama::LangResult;

fn compile(input: &str) -> LangResult<Term> {
    let ast = parse(input)?;
    let mir = MirTerm::from_ast(ast)?;
    ty_check(&mir)?;
    Ok(Term::from_mir(mir))
}

fn arithmetic(c: &mut Criterion) {
    let input = include_str!("../tests/eval/arithmetic.pj");
    let term = compile(input).unwrap();
    c.bench_function("arithmetic", |b| b.iter(|| evaluate(term.clone())));
}

fn factorial(c: &mut Criterion) {
    let input = include_str!("../tests/eval/factorial.pj");
    let term = compile(input).unwrap();
    c.bench_function("factorial", |b| b.iter(|| evaluate(term.clone())));
}

fn factorial_tail(c: &mut Criterion) {
    let input = include_str!("../tests/eval/factorial_tail.pj");
    let term = compile(input).unwrap();
    c.bench_function("factorial_tail", |b| b.iter(|| evaluate(term.clone())));
}

fn fibonacci(c: &mut Criterion) {
    let input = include_str!("../tests/eval/fibonacci.pj");
    let term = compile(input).unwrap();
    c.bench_function("fibonacci", |b| b.iter(|| evaluate(term.clone())));
}

fn fibonacci_tail(c: &mut Criterion) {
    let input = include_str!("../tests/eval/fibonacci_tail.pj");
    let term = compile(input).unwrap();
    c.bench_function("fibonacci_tail", |b| b.iter(|| evaluate(term.clone())));
}

fn fancy_max(c: &mut Criterion) {
    let input = include_str!("../tests/eval/fancy_max.pj");
    let term = compile(input).unwrap();
    c.bench_function("fancy_max", |b| b.iter(|| evaluate(term.clone())));
}

fn step(c: &mut Criterion) {
    let input = include_str!("../tests/eval/step.pj");
    let term = compile(input).unwrap();
    c.bench_function("step", |b| b.iter(|| evaluate(term.clone())));
}

criterion_group!(
    benches,
    arithmetic,
    factorial,
    factorial_tail,
    fibonacci,
    fibonacci_tail,
    fancy_max,
    step
);
criterion_main!(benches);
