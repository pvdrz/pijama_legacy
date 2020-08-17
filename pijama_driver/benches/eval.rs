use criterion::{criterion_group, criterion_main, Criterion};

use std::include_str;

use pijama_common::location::LocatedError;
use pijama_ctx::Context;
use pijama_driver::LangResult;
use pijama_parser::parse;
use pijama_tycheck::ty_check;
use pijama_vm::{CodeBuf, Machine, Heap};

fn compile<'code>(input: &str, code: &'code mut Vec<CodeBuf>, heap: &'code Heap) -> LangResult<Machine<'code>> {
    let mut ctx = Context::new();
    let ast = parse(input).map_err(LocatedError::kind_into)?;
    let hir = pijama_hir::lower_ast(&mut ctx, ast).map_err(LocatedError::kind_into)?;
    ty_check(&hir, &mut ctx).map_err(LocatedError::kind_into)?;
    let mir = pijama_mir::Term::from_hir(&hir, &mut ctx);
    let machine = pijama_codegen::compile(&ctx, code, heap, &mir);
    Ok(machine)
}

fn arithmetic(c: &mut Criterion) {
    let input = include_str!("arithmetic.pj");
    let mut code = vec![];
    let heap = Heap::new();
    let interpreter = compile(input, &mut code, &heap).unwrap();
    c.bench_function("arithmetic", |b| b.iter(|| interpreter.clone().run()));
}

fn logic(c: &mut Criterion) {
    let input = include_str!("logic.pj");
    let mut code = vec![];
    let heap = Heap::new();
    let interpreter = compile(input, &mut code, &heap).unwrap();
    c.bench_function("logic", |b| b.iter(|| interpreter.clone().run()));
}

fn factorial(c: &mut Criterion) {
    let input = include_str!("factorial.pj");
    let mut code = vec![];
    let heap = Heap::new();
    let interpreter = compile(input, &mut code, &heap).unwrap();
    c.bench_function("factorial", |b| b.iter(|| interpreter.clone().run()));
}

fn factorial_tail(c: &mut Criterion) {
    let input = include_str!("factorial_tail.pj");
    let mut code = vec![];
    let heap = Heap::new();
    let interpreter = compile(input, &mut code, &heap).unwrap();
    c.bench_function("factorial_tail", |b| b.iter(|| interpreter.clone().run()));
}

fn fibonacci(c: &mut Criterion) {
    let input = include_str!("fibonacci.pj");
    let mut code = vec![];
    let heap = Heap::new();
    let interpreter = compile(input, &mut code, &heap).unwrap();
    c.bench_function("fibonacci", |b| b.iter(|| interpreter.clone().run()));
}

fn fibonacci_tail(c: &mut Criterion) {
    let input = include_str!("fibonacci_tail.pj");
    let mut code = vec![];
    let heap = Heap::new();
    let interpreter = compile(input, &mut code, &heap).unwrap();
    c.bench_function("fibonacci_tail", |b| b.iter(|| interpreter.clone().run()));
}

fn fancy_max(c: &mut Criterion) {
    let input = include_str!("fancy_max.pj");
    let mut code = vec![];
    let heap = Heap::new();
    let interpreter = compile(input, &mut code, &heap).unwrap();
    c.bench_function("fancy_max", |b| b.iter(|| interpreter.clone().run()));
}

fn step(c: &mut Criterion) {
    let input = include_str!("step.pj");
    let mut code = vec![];
    let heap = Heap::new();
    let interpreter = compile(input, &mut code, &heap).unwrap();
    c.bench_function("step", |b| b.iter(|| interpreter.clone().run()));
}

fn gcd(c: &mut Criterion) {
    let input = include_str!("gcd.pj");
    let mut code = vec![];
    let heap = Heap::new();
    let interpreter = compile(input, &mut code, &heap).unwrap();
    c.bench_function("gcd", |b| b.iter(|| interpreter.clone().run()));
}

fn ackermann(c: &mut Criterion) {
    let input = include_str!("ackermann.pj");
    let mut code = vec![];
    let heap = Heap::new();
    let interpreter = compile(input, &mut code, &heap).unwrap();
    c.bench_function("ackermann", |b| b.iter(|| interpreter.clone().run()));
}

fn calling(c: &mut Criterion) {
    let input = include_str!("calling.pj");
    let mut code = vec![];
    let heap = Heap::new();
    let interpreter = compile(input, &mut code, &heap).unwrap();
    c.bench_function("calling", |b| b.iter(|| interpreter.clone().run()));
}

fn complex_calling(c: &mut Criterion) {
    let input = include_str!("complex_calling.pj");
    let mut code = vec![];
    let heap = Heap::new();
    let interpreter = compile(input, &mut code, &heap).unwrap();
    c.bench_function("complex_calling", |b| b.iter(|| interpreter.clone().run()));
}

fn cond_chain(c: &mut Criterion) {
    let input = include_str!("cond_chain.pj");
    let mut code = vec![];
    let heap = Heap::new();
    let interpreter = compile(input, &mut code, &heap).unwrap();
    c.bench_function("cond_chain", |b| b.iter(|| interpreter.clone().run()));
}

fn short_circuit(c: &mut Criterion) {
    let input = include_str!("short_circuit.pj");
    let mut code = vec![];
    let heap = Heap::new();
    let interpreter = compile(input, &mut code, &heap).unwrap();
    c.bench_function("short_circuit", |b| b.iter(|| interpreter.clone().run()));
}

fn adler32(c: &mut Criterion) {
    let input = include_str!("adler32.pj");
    let mut code = vec![];
    let heap = Heap::new();
    let interpreter = compile(input, &mut code, &heap).unwrap();
    c.bench_function("adler32", |b| b.iter(|| interpreter.clone().run()));
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
    // complex_calling,
    // fancy_max,
    // step,
    cond_chain,
    short_circuit,
    adler32,
);
criterion_main!(benches);
