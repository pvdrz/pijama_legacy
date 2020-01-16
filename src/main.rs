mod parser;

fn main() -> parser::ParseResult {
    dbg!(parser::parse("
    (defun map (option f)
      (case option
        ((Some t) (Some (f t)))
        (None None)))
    ")?);

    Ok(())
}
