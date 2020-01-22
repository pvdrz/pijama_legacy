mod parser;
mod ir;

use std::convert::TryFrom;

fn main() -> parser::ParseResult {
    let input = "(defun factorial (n) (switch n (0 1) (* n (factorial (- n 1)))))";

    let nodes = parser::parse(input)?;

    let _exprs = dbg!(nodes.iter().map(ir::Expr::try_from).collect::<Result<Vec<_>, _>>());

    Ok(())
}
