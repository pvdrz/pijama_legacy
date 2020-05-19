use super::Term;
use crate::ast::{BinOp, Literal, UnOp};

/// Evaluation step for beta reduction ((λ. body) arg)
#[inline(always)]
pub(super) fn step_beta_reduction(mut body: Box<Term>, mut arg: Box<Term>) -> (bool, Term) {
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

pub(super) fn native_bin_op(op: BinOp, l1: Literal, l2: Literal) -> Literal {
    use BinOp::*;
    use Literal::*;

    match (op, l1, l2) {
        (Add, Number(n1), Number(n2)) => (n1 + n2).into(),
        (Sub, Number(n1), Number(n2)) => (n1 - n2).into(),
        (Mul, Number(n1), Number(n2)) => (n1 * n2).into(),
        (Div, Number(n1), Number(n2)) => (n1 / n2).into(),
        (Rem, Number(n1), Number(n2)) => (n1 % n2).into(),
        (Lt, Number(n1), Number(n2)) => (n1 < n2).into(),
        (Lte, Number(n1), Number(n2)) => (n1 <= n2).into(),
        (Gt, Number(n1), Number(n2)) => (n1 > n2).into(),
        (Gte, Number(n1), Number(n2)) => (n1 >= n2).into(),
        (Eq, l1, l2) => (l1 == l2).into(),
        (Neq, l1, l2) => (l1 != l2).into(),
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

pub(super) fn native_un_op(op: UnOp, lit: Literal) -> Literal {
    use Literal::*;
    use UnOp::*;

    match (op, lit) {
        (Neg, Number(n)) => (-n).into(),
        (Not, Bool(b)) => (!b).into(),
        (op, lit) => panic!("Unexpected operation `{} {}`", op, lit),
    }
}
