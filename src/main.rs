mod parser;
mod ir;

fn main() -> parser::ParseResult {
    let input = "
    (defun map (option f)
      (case option
        ((Some t) (Some (f t)))
        (None None)))
    ";

    let input = "(foo + xy)";

    let nodes = parser::parse(input)?;

    let exprs = dbg!(nodes.into_iter().map(ir::Expr::from).collect::<Vec<_>>());

    Ok(())
}
