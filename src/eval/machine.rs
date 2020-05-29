use crate::{
    ast::{BinOp, Literal, Primitive, UnOp},
    eval::LangEnv,
    lir::Term::{self, *},
};

use std::{
    borrow::{Borrow, BorrowMut},
    io::Write,
};

pub trait Machine<W: Write>: Sized {
    fn with_env(env: LangEnv<W>) -> Self;

    fn lang_env(&mut self) -> &mut LangEnv<W>;

    fn evaluate(&mut self, mut term: Term) -> Term {
        while {
            let (eval, new_term) = self.step(term);
            term = new_term;
            eval
        } {}
        term
    }

    fn step(&mut self, term: Term) -> (bool, Term) {
        match term {
            // Dispatch step for binary operations
            BinaryOp(op, t1, t2) => self.step_bin_op(op, t1, t2),
            // Dispatch step for unary operations
            UnaryOp(op, t1) => self.step_un_op(op, t1),
            // Dispatch step for beta reduction
            // Application with unevaluated first term (t1 t2)
            // Evaluate t1.
            App(mut t1, arg) => match *t1 {
                Abs(body) => self.step_beta_reduction(*body, arg),
                PrimFn(prim) => self.step_primitive_app(prim, arg),
                _ => (self.step_in_place(t1.borrow_mut()), App(t1, arg)),
            },
            // Dispatch step for conditionals
            Cond(t1, t2, t3) => self.step_conditional(t1, t2, t3),
            // Dispatch step for fixed point operation
            Fix(t1) => self.step_fix(t1),
            // Any other term stops the evaluation.
            Var(_) | Lit(_) | Abs(_) | PrimFn(_) | Hole => (false, term),
        }
    }

    fn step_in_place(&mut self, term: &mut Term) -> bool {
        let inner_term = std::mem::replace(term, Hole);
        let (cont, inner_term) = self.step(inner_term);
        *term = inner_term;
        cont
    }

    /// Evaluation step for conditionals (if t1 then t2 else t3)
    fn step_conditional(
        &mut self,
        mut t1: Box<Term>,
        t2: Box<Term>,
        t3: Box<Term>,
    ) -> (bool, Term) {
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
            (self.step_in_place(t1.borrow_mut()), Term::Cond(t1, t2, t3))
        }
    }

    /// Evaluation step for binary operations (t1 op t2)
    fn step_bin_op(&mut self, op: BinOp, mut a: Box<Term>, mut b: Box<Term>) -> (bool, Term) {
        use BinOp::*;

        match (op, a.borrow(), b.borrow()) {
            // If op is && and t1 is false evaluate to false
            (And, Lit(0), _) => (true, false.into()),
            // If op is || and t1 is true evaluate to true
            (Or, Lit(1), _) => (true, true.into()),
            // If both are literals evaluate with native operation
            (_, Lit(l1), Lit(l2)) => (true, Lit(Self::native_bin_op(op, *l1, *l2))),
            // If t2 is not a literal, evaluate it.
            (_, Lit(_), _) => (self.step_in_place(b.borrow_mut()), Term::BinaryOp(op, a, b)),
            // If t1 is not a literal, evaluate it.
            _ => (self.step_in_place(a.borrow_mut()), Term::BinaryOp(op, a, b)),
        }
    }

    /// Evaluation step for unary operations (op t1)
    fn step_un_op(&mut self, op: UnOp, mut t1: Box<Term>) -> (bool, Term) {
        // If t1 is a literal, do the operation.
        if let Term::Lit(lit) = t1.borrow() {
            (true, Term::Lit(Self::native_un_op(op, *lit)))
        // If t1 is not a literal, evaluate it.
        } else {
            (self.step_in_place(&mut t1), Term::UnaryOp(op, t1))
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
            (self.step_in_place(&mut t1), Term::Fix(t1))
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
    fn step_primitive_app(&mut self, prim: Primitive, arg: Box<Term>) -> (bool, Term) {
        match prim {
            Primitive::Print => {
                writeln!(self.lang_env().stdout(), "{}", arg).expect("Primitive print failed");
                (true, Literal::Unit.into())
            }
        }
    }

    fn native_bin_op(op: BinOp, n1: i64, n2: i64) -> i64 {
        use BinOp::*;

        match op {
            Add => n1 + n2,
            Sub => n1 - n2,
            Mul => n1 * n2,
            Div => n1 / n2,
            Rem => n1 % n2,
            Lt => (n1 < n2).into(),
            Lte => (n1 <= n2).into(),
            Gt => (n1 > n2).into(),
            Gte => (n1 >= n2).into(),
            Eq => (n1 == n2).into(),
            Neq => (n1 != n2).into(),
            BitAnd | And => n1 & n2,
            BitOr | Or => n1 | n2,
            BitXor => n1 ^ n2,
            Shr => n1 >> n2,
            Shl => n1 << n2,
        }
    }

    fn native_un_op(op: UnOp, n: i64) -> i64 {
        use UnOp::*;

        match op {
            Neg => -n,
            Not => !n,
        }
    }
}
