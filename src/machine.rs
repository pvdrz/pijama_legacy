use crate::lir::Term;

mod eval;

#[derive(Default)]
pub struct Machine {}

impl Machine {
    pub fn evaluate(&mut self, mut term: Term) -> Term {
        while {
            let (eval, new_term) = self.step(term);
            term = new_term;
            eval
        } {}
        term
    }
}
