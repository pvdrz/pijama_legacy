use criterion::{criterion_group, criterion_main, Criterion};

use std::include_str;

use pijama::{
    lir::{evaluate, Term},
    mir::Term as MirTerm,
    parser::parse,
    ty::ty_check,
    LangResult,
};

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

fn logic(c: &mut Criterion) {
    let input = include_str!("../tests/eval/logic.pj");
    let term = compile(input).unwrap();
    c.bench_function("logic", |b| b.iter(|| evaluate(term.clone())));
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

fn gcd(c: &mut Criterion) {
    let input = include_str!("../tests/eval/gcd.pj");
    let term = compile(input).unwrap();
    c.bench_function("gcd", |b| b.iter(|| evaluate(term.clone())));
}

fn ackermann(c: &mut Criterion) {
    let input = include_str!("../tests/eval/ackermann.pj");
    let term = compile(input).unwrap();
    c.bench_function("ackermann", |b| b.iter(|| evaluate(term.clone())));
}

fn calling(c: &mut Criterion) {
    let input = include_str!("../tests/eval/calling.pj");
    let term = compile(input).unwrap();
    c.bench_function("calling", |b| b.iter(|| evaluate(term.clone())));
}

fn complex_calling(c: &mut Criterion) {
    let input = include_str!("../tests/eval/complex_calling.pj");
    let term = compile(input).unwrap();
    c.bench_function("complex_calling", |b| b.iter(|| evaluate(term.clone())));
}

criterion_group!(
    benches,
    arithmetic,
    logic,
    factorial,
    factorial_tail,
    fibonacci,
    fibonacci_tail,
    gcd,
    ackermann,
    calling,
    complex_calling,
    fancy_max,
    step
);
criterion_main!(benches);
