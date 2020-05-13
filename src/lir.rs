use std::fmt;

use crate::ast::*;

use Term::*;

mod ctx;

pub fn evaluate(term: Term) -> Term {
    term.evaluate()
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Term {
    Var(usize),
    Lit(Literal),
    Abs(Box<Term>),
    UnaryOp(UnOp, Box<Term>),
    BinaryOp(BinOp, Box<Term>, Box<Term>),
    App(Box<Term>, Box<Term>),
    Cond(Box<Term>, Box<Term>, Box<Term>),
    Fix(Box<Term>),
    Hole,
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Hole => write!(f, "hole"),
            Var(var) => write!(f, "_{}", var),
            Abs(term) => write!(f, "(λ. {})", term),
            UnaryOp(op, term) => write!(f, "({}{})", op, term),
            BinaryOp(op, t1, t2) => write!(f, "({} {} {})", t1, op, t2),
            App(t1, t2) => write!(f, "({} {})", t1, t2),
            Lit(literal) => write!(f, "{}", literal),
            Cond(t1, t2, t3) => write!(f, "(if {} then {} else {})", t1, t2, t3),
            Fix(t1) => write!(f, "(fix {})", t1),
        }
    }
}

impl Term {
    pub fn from_mir(mir: crate::mir::Term) -> Self {
        ctx::remove_names(mir)
    }

    fn shift(&mut self, up: bool, cutoff: usize) {
        match self {
            Lit(_) | Hole => (),
            Var(index) => {
                if *index >= cutoff {
                    if up {
                        *index += 1;
                    } else {
                        *index -= 1;
                    }
                }
            }
            Abs(body) => {
                body.shift(up, cutoff + 1);
            }
            UnaryOp(_, t1) => {
                t1.shift(up, cutoff);
            }
            BinaryOp(_, t1, t2) => {
                t1.shift(up, cutoff);
                t2.shift(up, cutoff);
            }
            App(t1, t2) => {
                t1.shift(up, cutoff);
                t2.shift(up, cutoff);
            }
            Cond(t1, t2, t3) => {
                t1.shift(up, cutoff);
                t2.shift(up, cutoff);
                t3.shift(up, cutoff);
            }
            Fix(t1) => {
                t1.shift(up, cutoff);
            }
        }
    }

    fn replace(&mut self, index: usize, subs: &mut Term) {
        match self {
            Lit(_) | Hole => (),
            Var(index2) => {
                if index == *index2 {
                    *self = subs.clone();
                }
            }
            Abs(body) =>  {
                    subs.shift(true, 0);
                    body.replace(index + 1, subs);
                    subs.shift(false, 0);
            },
            UnaryOp(_, t1) => {
                t1.replace(index, subs);
            }
            BinaryOp(_, t1, t2) => {
                t1.replace(index, subs);
                t2.replace(index, subs);
            }
            App(t1, t2) => {
                t1.replace(index, subs);
                t2.replace(index, subs);
            }
            Cond(t1, t2, t3) => {
                t1.replace(index, subs);
                t2.replace(index, subs);
                t3.replace(index, subs);
            }
            Fix(t1) => {
                t1.replace(index, subs);
            }
        }
    }

    fn step_in_place(&mut self) -> bool {
        let term = std::mem::replace(self, Hole);
        let (cont, term) = term.step();
        *self = term;
        cont
    }

    fn step(mut self) -> (bool, Term) {
        match self {
            // Binary operations (t1 op t2)
            // If t1 and t2 are literals, do the operation.
            BinaryOp(op, box Lit(l1), box Lit(l2)) => {
                (true, Lit(eval_bin_op(op, l1, l2)))
            }
            // If t2 is not a literal, evaluate it.
            BinaryOp(_, box Lit(_), ref mut t2) => (t2.step_in_place(), self),
            // If t1 is not a literal, evaluate it.
            BinaryOp(_, ref mut t1, _) => (t1.step_in_place(), self),

            // Beta Reduction ((\. b) t2)
            // Replace the argument of the function by t2 inside b and evaluate to b.
            App(box Abs(box mut body), box mut t2) => {
                t2.shift(true, 0);
                body.replace(0, &mut t2);
                body.shift(false, 0);
                (true, body)
            }

            // Unary Operations (op t1)
            // If t1 is a literal, do the operation.
            UnaryOp(op, box Lit(lit)) => (true, Lit(eval_un_op(op, lit))),
            // If t1 is not a literal, evaluate it.
            UnaryOp(_, ref mut t1) => (t1.step_in_place(), self),

            // Application with unevaluated first term (t1 t2)
            // Evaluate t1.
            App(ref mut t1, _) => (t1.step_in_place(), self),

            // Conditionals (if t1 then t2 else t3)
            // If t1 is true, evaluate to t2.
            Cond(box Lit(Literal::True), box t2, _) => (true, t2),
            // If t1 is false, evaluate to t3.
            Cond(box Lit(Literal::False), _, box t3) => (true, t3),
            // If t1 is any other literal, this is an error.
            Cond(box Lit(lit), _, _) => panic!("Found non-boolean literal {} in condition", lit),
            // If t1 is not a literal, evaluate it.
            Cond(ref mut t1, _, _) => (t1.step_in_place(), self),

            // Fixed-point operation (fix t1)
            // If t1 is an abstraction (\. t2), replace the argument of t1 by (fix t1) inside t2
            // and evaluate to t2.
            Fix(box Abs(box ref t2)) => {
                let mut t2 = t2.clone();
                t2.replace(0, &mut self);
                (true, t2)
            }
            // If t1 is not an abstraction, evaluate it.
            Fix(ref mut t1) => (t1.step_in_place(), self),

            // Any other term stops the evaluation.
            Var(_) | Lit(_) | Abs(_) | Hole => (false, self),
        }
    }

    fn evaluate(self) -> Term {
        let mut term = self;
        while {
            let (eval, new_term) = term.step();
            term = new_term;
            eval
        } {}
        term
    }
}

fn eval_bin_op(op: BinOp, l1: Literal, l2: Literal) -> Literal {
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
        (And, True, True) => True,
        (And, True, False) => False,
        (And, False, True) => False,
        (And, False, False) => False,
        (Or, True, True) => True,
        (Or, True, False) => True,
        (Or, False, True) => True,
        (Or, False, False) => False,
        (op, l1, l2) => panic!("Unexpected operation `{} {} {}`", l1, op, l2),
    }
}

fn eval_un_op(op: UnOp, lit: Literal) -> Literal {
    use Literal::*;
    use UnOp::*;
    match (op, lit) {
        (Minus, Number(n)) => Number(-n),
        (Not, True) => False,
        (Not, False) => True,
        (op, lit) => panic!("Unexpected operation `{} {}`", op, lit),
    }
}
