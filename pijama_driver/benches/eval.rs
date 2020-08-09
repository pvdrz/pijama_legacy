use criterion::{criterion_group, criterion_main, Criterion};

use std::include_str;

use pijama_parser::parse;
use pijama_tycheck::ty_check;
use pijama_lir::Term;
use pijama_machine::MachineBuilder;
use pijama_driver::LangResult;
use pijama_common::location::LocatedError;

fn compile(input: &str) -> LangResult<Term> {
    let ast = parse(input).map_err(LocatedError::kind_into)?;
    let (hir, ctx) = pijama_hir::lower_ast(ast).map_err(LocatedError::kind_into)?;
    ty_check(&hir, ctx).map_err(LocatedError::kind_into)?;
    Ok(Term::from_hir(hir))
}

fn arithmetic(c: &mut Criterion) {
    let input = include_str!("arithmetic.pj");
    let term = compile(input).unwrap();
    let mut machine = MachineBuilder::default().build();
    c.bench_function("arithmetic", |b| b.iter(|| machine.evaluate(term.clone())));
}

fn logic(c: &mut Criterion) {
    let input = include_str!("logic.pj");
    let term = compile(input).unwrap();
    let mut machine = MachineBuilder::default().build();
    c.bench_function("logic", |b| b.iter(|| machine.evaluate(term.clone())));
}

fn factorial(c: &mut Criterion) {
    let input = include_str!("factorial.pj");
    let term = compile(input).unwrap();
    let mut machine = MachineBuilder::default().build();
    c.bench_function("factorial", |b| b.iter(|| machine.evaluate(term.clone())));
}

fn factorial_tail(c: &mut Criterion) {
    let input = include_str!("factorial_tail.pj");
    let term = compile(input).unwrap();
    let mut machine = MachineBuilder::default().build();
    c.bench_function("factorial_tail", |b| {
        b.iter(|| machine.evaluate(term.clone()))
    });
}

fn fibonacci(c: &mut Criterion) {
    let input = include_str!("fibonacci.pj");
    let term = compile(input).unwrap();
    let mut machine = MachineBuilder::default().build();
    c.bench_function("fibonacci", |b| b.iter(|| machine.evaluate(term.clone())));
}

fn fibonacci_tail(c: &mut Criterion) {
    let input = include_str!("fibonacci_tail.pj");
    let term = compile(input).unwrap();
    let mut machine = MachineBuilder::default().build();
    c.bench_function("fibonacci_tail", |b| {
        b.iter(|| machine.evaluate(term.clone()))
    });
}

fn fancy_max(c: &mut Criterion) {
    let input = include_str!("fancy_max.pj");
    let term = compile(input).unwrap();
    let mut machine = MachineBuilder::default().build();
    c.bench_function("fancy_max", |b| b.iter(|| machine.evaluate(term.clone())));
}

fn step(c: &mut Criterion) {
    let input = include_str!("step.pj");
    let term = compile(input).unwrap();
    let mut machine = MachineBuilder::default().build();
    c.bench_function("step", |b| b.iter(|| machine.evaluate(term.clone())));
}

fn gcd(c: &mut Criterion) {
    let input = include_str!("gcd.pj");
    let term = compile(input).unwrap();
    let mut machine = MachineBuilder::default().build();
    c.bench_function("gcd", |b| b.iter(|| machine.evaluate(term.clone())));
}

fn ackermann(c: &mut Criterion) {
    let input = include_str!("ackermann.pj");
    let term = compile(input).unwrap();
    let mut machine = MachineBuilder::default().build();
    c.bench_function("ackermann", |b| b.iter(|| machine.evaluate(term.clone())));
}

fn calling(c: &mut Criterion) {
    let input = include_str!("calling.pj");
    let term = compile(input).unwrap();
    let mut machine = MachineBuilder::default().build();
    c.bench_function("calling", |b| b.iter(|| machine.evaluate(term.clone())));
}

fn complex_calling(c: &mut Criterion) {
    let input = include_str!("complex_calling.pj");
    let term = compile(input).unwrap();
    let mut machine = MachineBuilder::default().build();
    c.bench_function("complex_calling", |b| {
        b.iter(|| machine.evaluate(term.clone()))
    });
}

fn cond_chain(c: &mut Criterion) {
    let input = include_str!("cond_chain.pj");
    let term = compile(input).unwrap();
    let mut machine = MachineBuilder::default().build();
    c.bench_function("cond_chain", |b| b.iter(|| machine.evaluate(term.clone())));
}

fn short_circuit(c: &mut Criterion) {
    let input = include_str!("short_circuit.pj");
    let term = compile(input).unwrap();
    let mut machine = MachineBuilder::default().build();
    c.bench_function("short_circuit", |b| {
        b.iter(|| machine.evaluate(term.clone()))
    });
}

fn adler32(c: &mut Criterion) {
    let input = include_str!("adler32.pj");
    let term = compile(input).unwrap();
    let mut machine = MachineBuilder::default().build();
    c.bench_function("adler32", |b| b.iter(|| machine.evaluate(term.clone())));
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
    step,
    cond_chain,
    short_circuit,
    adler32,
);
criterion_main!(benches);
