use crate::ast::{BinOp, BuiltInFn, Literal, UnOp};
use crate::lir::Term::{self, *};
use crate::LangEnv;
use eval::*;

mod eval;

pub struct Machine<'a, 'b> {
    pub env: &'a mut LangEnv<'b>,
}

impl<'a, 'b> Machine<'a, 'b> {
    pub fn evaluate(&mut self, mut term: Term) -> Term {
        while {
            let (eval, new_term) = self.step(term);
            term = new_term;
            eval
        } {}
        term
    }

    fn step(&mut self, mut term: Term) -> (bool, Term) {
        match term {
            // Dispatch step for binary operations
            BinaryOp(op, t1, t2) => self.step_bin_op(op, t1, t2),
            // Dispatch step for unary operations
            UnaryOp(op, t1) => self.step_un_op(op, t1),
            // Dispatch step for beta reduction
            App(box Abs(body), arg) => step_beta_reduction(body, arg),
            // Builtin function special handling necessary
            App(box Term::BuiltInFn(BuiltInFn::Print), ref t1) => {
                writeln!(self.env.stdout, "{}", t1).expect("Print failed");
                (true, Term::Lit(Literal::Unit))
            }
            // Application with unevaluated first term (t1 t2)
            // Evaluate t1.
            App(ref mut t1, _) => (self.step_in_place(t1), term),
            // Dispatch step for conditionals
            Cond(t1, t2, t3) => self.step_conditional(t1, t2, t3),
            // Dispatch step for fixed point operation
            Fix(t1) => self.step_fix(t1),
            // Any other term stops the evaluation.
            Var(_) | Lit(_) | Abs(_) | BuiltInFn(_) | Hole => (false, term),
        }
    }

    fn step_in_place(&mut self, term: &mut Term) -> bool {
        let inner_term = std::mem::replace(term, Hole);
        let (cont, inner_term) = self.step(inner_term);
        *term = inner_term;
        cont
    }

    /// Evaluation step for conditionals (if t1 then t2 else t3)
    #[inline(always)]
    pub fn step_conditional(
        &mut self,
        mut t1: Box<Term>,
        t2: Box<Term>,
        t3: Box<Term>,
    ) -> (bool, Term) {
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
            (self.step_in_place(t1.as_mut()), Term::Cond(t1, t2, t3))
        }
    }

    /// Evaluation step for binary operations (t1 op t2)
    #[inline(always)]
    pub fn step_bin_op(&mut self, op: BinOp, t1: Box<Term>, t2: Box<Term>) -> (bool, Term) {
        use BinOp::*;
        use Literal::*;

        match (op, t1, t2) {
            (op, box Lit(l1), box Lit(l2)) => (true, Lit(native_bin_op(op, l1, l2))),
            // If op is && and t1 is false evaluate to false
            (And, box Lit(Bool(false)), _) => (true, Lit(Bool(false))),
            // If op is || and t1 is true evaluate to true
            (Or, box Lit(Bool(true)), _) => (true, Lit(Bool(true))),
            // If t2 is not a literal, evaluate it.
            (op, t1 @ box Lit(_), mut t2) => {
                (self.step_in_place(t2.as_mut()), Term::BinaryOp(op, t1, t2))
            }
            // If t1 is not a literal, evaluate it.
            (op, mut t1, t2) => (self.step_in_place(t1.as_mut()), Term::BinaryOp(op, t1, t2)),
        }
    }

    /// Evaluation step for unary operations (op t1)
    #[inline(always)]
    pub fn step_un_op(&mut self, op: UnOp, mut t1: Box<Term>) -> (bool, Term) {
        // If t1 is a literal, do the operation.
        if let box Term::Lit(lit) = t1 {
            (true, Term::Lit(native_un_op(op, lit)))
        // If t1 is not a literal, evaluate it.
        } else {
            (self.step_in_place(t1.as_mut()), Term::UnaryOp(op, t1))
        }
    }

    /// Evaluation step for the fixed-point operation (fix t1)
    pub fn step_fix(&mut self, mut t1: Box<Term>) -> (bool, Term) {
        // If t1 is an abstraction (\. t2), replace the argument of t1 by (fix t1) inside t2
        // and evaluate to t2.
        if let box Term::Abs(box ref t2) = t1 {
            let mut t2 = t2.clone();
            t2.replace(0, &mut Term::Fix(t1));
            (true, t2)
        // If t1 is not an abstraction, evaluate it.
        } else {
            (self.step_in_place(t1.as_mut()), Term::Fix(t1))
        }
    }
}
