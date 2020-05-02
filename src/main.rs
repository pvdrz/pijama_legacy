mod ast;
mod ctx;
mod lir;
mod mir;
mod parser;
mod ty;

use parser::parse;

fn main() {
    let input = include_str!("source.pj");
    let ast = parse(input).unwrap();
    let mir = mir::lower(ast);
    println!("{}", mir);
    let ty = ty::ty_check(&mir);
    println!("{}", ty);
    let mut lir = ctx::remove_names(mir);
    println!("{}", lir);
    lir.evaluate();
    println!("{}", lir);
}
