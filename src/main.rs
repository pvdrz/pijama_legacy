mod ast;
mod eval;
mod ir;
mod lower;
mod parser;
mod ty;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "
    (defun fact (n)
      ((defun aux (n acc)
         (if (= n 0)
             acc
             (aux (- n 1) (* acc n))))
       n 1))
   (/ (fact 5) (fact 4))
        ";

    let nodes = dbg!(parser::parse(input)?);

    let mut ctx = lower::Context::default();
    let mut interpreter = eval::Interpreter::default();

    for node in nodes {
        let expr = dbg!(ctx.lower(&node));
        dbg!(interpreter.eval(&expr.unwrap()));
    }

    Ok(())
}
