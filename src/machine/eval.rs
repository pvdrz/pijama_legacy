use pijama_ast::{BinOp, Literal, Primitive, UnOp};

use crate::{
    lir::Term::{self, *},
    machine::{arithmetic::Arithmetic, Machine},
};

use std::{borrow::Borrow, io::Write};

impl<W: Write, A: Arithmetic> Machine<W, A> {
    pub(super) fn eval(&mut self, mut term: Term) -> (bool, Term) {
        let mut changed = false;
        while {
            let (eval, new_term) = self.step(term);
            term = new_term;
            eval
        } {
            changed = true;
        }
        (changed, term)
    }

    pub(super) fn step(&mut self, term: Term) -> (bool, Term) {
        match term {
            // Dispatch step for binary operations
            BinaryOp(op, t1, t2) => self.step_bin_op(op, t1, t2),
            // Dispatch step for unary operations
            UnaryOp(op, t1) => self.step_un_op(op, t1),
            App(mut t1, arg) => match *t1 {
                // Dispatch step for beta reduction
                Abs(body) => self.step_beta_reduction(*body, arg),
                // Dispatch step for primitive application
                PrimFn(prim) => self.step_primitive_app(prim, *arg),
                // Application with unevaluated first term (t1 t2)
                // Evaluate t1.
                _ => {
                    let (changed, new_t1) = self.eval(*t1);
                    *t1 = new_t1;
                    (changed, App(t1, arg))
                }
            },
            // Dispatch step for conditionals
            Cond(t1, t2, t3) => self.step_cond(t1, t2, t3),
            // Dispatch step for fixed point operation
            Fix(t1) => self.step_fix(t1),
            // Any other term stops the evaluation.
            Var(_) | Lit(_) | Abs(_) | PrimFn(_) | Hole => (false, term),
        }
    }
    /// Evaluation step for conditionals (if t1 then t2 else t3)
    fn step_cond(&mut self, mut t1: Box<Term>, t2: Box<Term>, t3: Box<Term>) -> (bool, Term) {
        // If t1 is a literal, we should be able to evaluate the conditional
        if let lit @ Term::Lit(_) = t1.borrow() {
            if lit.as_bool() {
                // If t1 is true, evaluate to t2.
                (true, *t2)
            } else {
                // If t1 is false, evaluate to t3.
                (true, *t3)
            }
        } else {
            // If t1 is not a literal, evaluate it in place and return (if t1 then t2 else t3)
            let (changed, new_t1) = self.eval(*t1);
            *t1 = new_t1;
            (changed, Term::Cond(t1, t2, t3))
        }
    }

    /// Evaluation step for binary operations (t1 op t2)
    fn step_bin_op(&mut self, op: BinOp, mut t1: Box<Term>, mut t2: Box<Term>) -> (bool, Term) {
        use BinOp::*;

        match (op, t1.borrow(), t2.borrow()) {
            // If op is && and t1 is false evaluate to false
            (And, Lit(0), _) => (true, false.into()),
            // If op is || and t1 is true evaluate to true
            (Or, Lit(1), _) => (true, true.into()),
            // If both are literals evaluate with native operation
            (_, Lit(l1), Lit(l2)) => (true, Lit(A::binary_operation(op, *l1, *l2))),
            // If t2 is not a literal, evaluate it.
            (_, Lit(_), _) => {
                let (changed, new_t2) = self.eval(*t2);
                *t2 = new_t2;
                (changed, Term::BinaryOp(op, t1, t2))
            }
            // If t1 is not a literal, evaluate it.
            _ => {
                let (changed, new_t1) = self.eval(*t1);
                *t1 = new_t1;
                (changed, Term::BinaryOp(op, t1, t2))
            }
        }
    }

    /// Evaluation step for unary operations (op t1)
    fn step_un_op(&mut self, op: UnOp, mut t1: Box<Term>) -> (bool, Term) {
        // If t1 is a literal, do the operation.
        if let Term::Lit(lit) = t1.borrow() {
            (true, Term::Lit(A::unary_operation(op, *lit)))
        // If t1 is not a literal, evaluate it.
        } else {
            let (changed, new_t1) = self.eval(*t1);
            *t1 = new_t1;
            (changed, Term::UnaryOp(op, t1))
        }
    }

    /// Evaluation step for the fixed-point operation (fix t1)
    fn step_fix(&mut self, mut t1: Box<Term>) -> (bool, Term) {
        // If t1 is an abstraction (\. t2), replace the argument of t1 by (fix t1) inside t2
        // and evaluate to t2.
        if let Term::Abs(t2) = t1.borrow() {
            let mut t2 = t2.clone();
            t2.replace(0, &mut Term::Fix(t1));
            (true, *t2)
        // If t1 is not an abstraction, evaluate it.
        } else {
            let (changed, new_t1) = self.eval(*t1);
            *t1 = new_t1;
            (changed, Term::Fix(t1))
        }
    }

    /// Evaluation step for beta reduction ((λ. body) arg)
    fn step_beta_reduction(&mut self, mut body: Term, mut arg: Box<Term>) -> (bool, Term) {
        // increase the indices of the argument so they can coincide with the indices of the body.
        arg.shift(true, 0);
        // replace the index 0 by the argument inside the body.
        body.replace(0, &mut arg);
        // decrease the indices of the body to take into account the fact that the abstraction no
        // longer exists.
        body.shift(false, 0);
        // return the body
        (true, body)
    }
    /// Evaluation step for application of primitive functions (prim arg)
    fn step_primitive_app(&mut self, prim: Primitive, arg: Term) -> (bool, Term) {
        match prim {
            Primitive::Print => {
                // Evaluate argument
                let (_, arg) = self.eval(arg);
                writeln!(self.env.stdout(), "{}", arg).expect("Primitive print failed");
                (true, Literal::Unit.into())
            }
        }
    }
}
