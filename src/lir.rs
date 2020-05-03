use std::fmt;

use crate::ast::*;

#[derive(Debug, Clone)]
pub enum Abstraction {
    Lambda(Box<Term>),
    Binary(BinOp),
    Unary(UnOp),
}

impl fmt::Display for Abstraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Abstraction::*;
        match self {
            Lambda(term) => write!(f, "(λ. {})", term),
            Binary(bin_op) => write!(f, "{}", bin_op),
            Unary(un_op) => write!(f, "{}", un_op),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Term {
    Var(usize),
    Lit(Literal),
    Abs(Abstraction),
    App(Box<Term>, Box<Term>),
    Cond(Box<Term>, Box<Term>, Box<Term>),
    Fix(Box<Term>),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(var) => write!(f, "_{}", var),
            Term::Abs(abs) => write!(f, "{}", abs),
            Term::App(t1, t2) => write!(f, "({} {})", t1, t2),
            Term::Lit(literal) => write!(f, "{}", literal),
            Term::Cond(t1, t2, t3) => write!(f, "(if {} then {} else {})", t1, t2, t3),
            Term::Fix(t1) => write!(f, "(fix {})", t1),
        }
    }
}

impl Term {
    fn is_value(&self) -> bool {
        match self {
            Term::Abs(_) | Term::Lit(_) => true,
            Term::App(box Term::Abs(Abstraction::Binary(_)), box v) if v.is_value() => true,
            _ => false,
        }
    }

    fn shift(&mut self, up: bool, cutoff: usize) {
        match self {
            Term::Lit(_) => (),
            Term::Var(index) => {
                if *index >= cutoff {
                    if up {
                        *index += 1;
                    } else {
                        *index -= 1;
                    }
                }
            }
            Term::Abs(abs) => match abs {
                Abstraction::Lambda(body) => {
                    body.shift(up, cutoff + 1);
                }
                Abstraction::Unary(_) | Abstraction::Binary(_) => (),
            },
            Term::App(t1, t2) => {
                t1.shift(up, cutoff);
                t2.shift(up, cutoff);
            }
            Term::Cond(t1, t2, t3) => {
                t1.shift(up, cutoff);
                t2.shift(up, cutoff);
                t3.shift(up, cutoff);
            }
            Term::Fix(t1) => {
                t1.shift(up, cutoff);
            }
        }
    }

    fn replace(&mut self, index: usize, subs: &mut Term) {
        match self {
            Term::Lit(_) => (),
            Term::Var(index2) => {
                if index == *index2 {
                    *self = subs.clone();
                }
            }
            Term::Abs(abs) => match abs {
                Abstraction::Lambda(body) => {
                    subs.shift(true, 0);
                    body.replace(index + 1, subs);
                    subs.shift(false, 0);
                }
                Abstraction::Unary(_) | Abstraction::Binary(_) => (),
            },
            Term::App(t1, t2) => {
                t1.replace(index, subs);
                t2.replace(index, subs);
            }
            Term::Cond(t1, t2, t3) => {
                t1.replace(index, subs);
                t2.replace(index, subs);
                t3.replace(index, subs);
            }
            Term::Fix(t1) => {
                t1.replace(index, subs);
            }
        }
    }

    fn step(&mut self) -> bool {
        match self {
            Term::Var(_) | Term::Lit(_) => false,
            Term::App(
                box Term::App(box Term::Abs(Abstraction::Binary(op)), box Term::Lit(l1)),
                box t2,
            ) => {
                if let Term::Lit(l2) = t2 {
                    if let Some(lit) = eval_bin_op(op, l1, l2) {
                        *self = Term::Lit(lit);
                        true
                    } else {
                        false
                    }
                } else {
                    t2.step()
                }
            }
            Term::App(box Term::Abs(abs), box t2) => match abs {
                Abstraction::Lambda(body) => {
                    t2.shift(true, 0);
                    body.replace(0, t2);
                    body.shift(false, 0);
                    *self = *body.clone();
                    true
                }
                Abstraction::Unary(op) => {
                    if let Term::Lit(l2) = t2 {
                        if let Some(lit) = eval_un_op(op, l2) {
                            *self = Term::Lit(lit);
                            true
                        } else {
                            false
                        }
                    } else {
                        t2.step()
                    }
                }
                Abstraction::Binary(_) => t2.step(),
            },
            Term::App(t1, _) => t1.step(),
            Term::Cond(box v1, box t2, box t3) if v1.is_value() => match v1 {
                Term::Lit(Literal::True) => {
                    *self = t2.clone();
                    true
                }
                Term::Lit(Literal::False) => {
                    *self = t3.clone();
                    true
                }
                _ => false,
            },
            Term::Cond(t1, _, _) => t1.step(),
            Term::Fix(t1) => match t1.as_ref() {
                Term::Abs(Abstraction::Lambda(box t2)) => {
                    let mut fix = Term::Fix(t1.clone());
                    let mut t2 = t2.clone();
                    t2.replace(0, &mut fix);
                    *self = t2;
                    true
                }
                _ => t1.step(),
            },
            _ => false,
        }
    }

    pub fn evaluate(&mut self) {
        while self.step() {}
    }
}

fn eval_bin_op(op: &BinOp, l1: &Literal, l2: &Literal) -> Option<Literal> {
    use BinOp::*;
    use Literal::*;
    match (op, l1, l2) {
        (Plus, Number(n1), Number(n2)) => Some((n1 + n2).into()),
        (Minus, Number(n1), Number(n2)) => Some((n1 - n2).into()),
        (Times, Number(n1), Number(n2)) => Some((n1 * n2).into()),
        (Divide, Number(n1), Number(n2)) => Some((n1 / n2).into()),
        (Modulo, Number(n1), Number(n2)) => Some((n1 % n2).into()),
        (LessThan, Number(n1), Number(n2)) => Some((n1 < n2).into()),
        (LessThanOrEqual, Number(n1), Number(n2)) => Some((n1 <= n2).into()),
        (GreaterThan, Number(n1), Number(n2)) => Some((n1 > n2).into()),
        (GreaterThanOrEqual, Number(n1), Number(n2)) => Some((n1 >= n2).into()),
        (Equal, _, _) => Some((l1 == l2).into()),
        (NotEqual, _, _) => Some((l1 != l2).into()),
        (And, True, True) => Some(True),
        (And, True, False) => Some(False),
        (And, False, True) => Some(False),
        (And, False, False) => Some(False),
        (Or, True, True) => Some(True),
        (Or, True, False) => Some(True),
        (Or, False, True) => Some(True),
        (Or, False, False) => Some(False),
        _ => None,
    }
}

fn eval_un_op(op: &UnOp, l1: &Literal) -> Option<Literal> {
    use Literal::*;
    use UnOp::*;
    match (op, l1) {
        (Minus, Number(n)) => Some(Number(-*n)),
        (Not, True) => Some(Literal::False),
        (Not, False) => Some(Literal::True),
        _ => None,
    }
}
