use super::Term;
use crate::ast::Literal;

/// Evaluation step for conditionals (if t1 then t2 else t3)
#[inline(always)]
pub fn step_conditional(mut t1: Box<Term>, t2: Box<Term>, t3: Box<Term>) -> (bool, Term) {
    // If t1 is a literal, we should be able to evaluate the conditional
    if let box Term::Lit(lit) = t1 {
        match lit {
            // If t1 is true, evaluate to t2.
            Literal::Bool(true) => (true, *t2),
            // If t1 is false, evaluate to t3.
            Literal::Bool(false) => (true, *t3),
            // If t1 is any other literal, panic
            lit => panic!("Found non-boolean literal {} in condition", lit),
        }
    // If t1 is not a literal, evaluate it in place and return (if t1 then t2 else t3)
    } else {
        (t1.step_in_place(), Term::Cond(t1, t2, t3))
    }
}
