mod parser;
mod ast;
mod mir;
mod lir;
mod ctx;

use parser::parse;

fn main() {
    let input = include_str!("source.pj");
    let ast = parse(input).unwrap();
    let mir = mir::compile_block(ast);
    println!("{}", mir);
    let mut lir = ctx::remove_names(mir);
    println!("{}", lir);
    lir.evaluate();
    println!("{}", lir);
}
