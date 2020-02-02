mod ast;
mod ir;
mod lower;
mod parser;
mod ty;

use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "
(defun fact (n)
  ((defun aux (n acc)
     (if (= n 0)
	     1
	     (aux (- n 1) (* acc n))))
   n 1))
        ";

    let nodes = dbg!(parser::parse(input)?);
    for node in nodes {
        let _expr = dbg!(lower::Context::default().lower(&node));
    }

    Ok(())
}
