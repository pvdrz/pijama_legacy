use super::Term;
use crate::ast::{BinOp, Literal, UnOp};

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

/// Evaluation step for beta reduction ((λ. body) arg)
#[inline(always)]
pub fn step_beta_reduction(mut body: Box<Term>, mut arg: Box<Term>) -> (bool, Term) {
    // increase the indices of the argument so they can coincide with the indices of the body.
    arg.shift(true, 0);
    // replace the index 0 by the argument inside the body.
    body.replace(0, &mut arg);
    // decrease the indices of the body to take into account the fact that the abstraction no
    // longer exists.
    body.shift(false, 0);
    // return the body
    (true, *body)
}

/// Evaluation step for binary operations (t1 op t2)
#[inline(always)]
pub fn step_bin_op(op: BinOp, t1: Box<Term>, t2: Box<Term>) -> (bool, Term) {
    use BinOp::*;
    use Literal::*;
    use Term::Lit;

    match (op, t1, t2) {
        (op, box Lit(l1), box Lit(l2)) => (true, Lit(native_bin_op(op, l1, l2))),
        // If op is && and t1 is false evaluate to false
        (And, box Lit(Bool(false)), _) => (true, Lit(Bool(false))),
        // If op is || and t1 is true evaluate to true
        (Or, box Lit(Bool(true)), _) => (true, Lit(Bool(true))),
        // If t2 is not a literal, evaluate it.
        (op, t1 @ box Lit(_), mut t2) => (t2.step_in_place(), Term::BinaryOp(op, t1, t2)),
        // If t1 is not a literal, evaluate it.
        (op, mut t1, t2) => (t1.step_in_place(), Term::BinaryOp(op, t1, t2)),
    }
}

/// Evaluation step for unary operations (op t1)
#[inline(always)]
pub fn step_un_op(op: UnOp, mut t1: Box<Term>) -> (bool, Term) {
    // If t1 is a literal, do the operation.
    if let box Term::Lit(lit) = t1 {
        (true, Term::Lit(native_un_op(op, lit)))
    // If t1 is not a literal, evaluate it.
    } else {
        (t1.step_in_place(), Term::UnaryOp(op, t1))
    }
}

/// Evaluation step for the fixed-point operation (fix t1)
pub fn step_fix(mut t1: Box<Term>) -> (bool, Term) {
    // If t1 is an abstraction (\. t2), replace the argument of t1 by (fix t1) inside t2
    // and evaluate to t2.
    if let box Term::Abs(box ref t2) = t1 {
        let mut t2 = t2.clone();
        t2.replace(0, &mut Term::Fix(t1));
        (true, t2)
    // If t1 is not an abstraction, evaluate it.
    } else {
        (t1.step_in_place(), Term::Fix(t1))
    }
}

fn native_bin_op(op: BinOp, l1: Literal, l2: Literal) -> Literal {
    use BinOp::*;
    use Literal::*;

    match (op, l1, l2) {
        (Plus, Number(n1), Number(n2)) => (n1 + n2).into(),
        (Minus, Number(n1), Number(n2)) => (n1 - n2).into(),
        (Times, Number(n1), Number(n2)) => (n1 * n2).into(),
        (Divide, Number(n1), Number(n2)) => (n1 / n2).into(),
        (Modulo, Number(n1), Number(n2)) => (n1 % n2).into(),
        (LessThan, Number(n1), Number(n2)) => (n1 < n2).into(),
        (LessThanOrEqual, Number(n1), Number(n2)) => (n1 <= n2).into(),
        (GreaterThan, Number(n1), Number(n2)) => (n1 > n2).into(),
        (GreaterThanOrEqual, Number(n1), Number(n2)) => (n1 >= n2).into(),
        (Equal, l1, l2) => (l1 == l2).into(),
        (NotEqual, l1, l2) => (l1 != l2).into(),
        (And, Bool(b1), Bool(b2)) => (b1 && b2).into(),
        (Or, Bool(b1), Bool(b2)) => (b1 || b2).into(),
        (BitAnd, Number(n1), Number(n2)) => (n1 & n2).into(),
        (BitOr, Number(n1), Number(n2)) => (n1 | n2).into(),
        (BitXor, Number(n1), Number(n2)) => (n1 ^ n2).into(),
        (Shr, Number(n1), Number(n2)) => (n1 >> n2).into(),
        (Shl, Number(n1), Number(n2)) => (n1 << n2).into(),
        (op, l1, l2) => panic!("Unexpected operation `{} {} {}`", l1, op, l2),
    }
}

fn native_un_op(op: UnOp, lit: Literal) -> Literal {
    use Literal::*;
    use UnOp::*;

    match (op, lit) {
        (Minus, Number(n)) => (-n).into(),
        (Not, Bool(b)) => (!b).into(),
        (op, lit) => panic!("Unexpected operation `{} {}`", op, lit),
    }
}
