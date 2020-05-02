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
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(var) => write!(f, "_{}", var),
            Term::Abs(abs) => write!(f, "{}", abs),
            Term::App(t1, t2) => write!(f, "({} {})", t1, t2),
            Term::Lit(literal) => write!(f, "{}", literal),
            Term::Cond(t1, t2, t3) => write!(f, "(if {} then {} else {})", t1, t2, t3),
        }
    }
}

impl Term {
    fn is_value(&self) -> bool {
        match self {
            Term::Abs(_) | Term::Lit(_) => true,
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
        }
    }

    fn step(&mut self) -> bool {
        match self {
            Term::Var(_) | Term::Lit(_) => false,
            Term::App(
                box Term::App(box Term::Abs(Abstraction::Binary(op)), box Term::Lit(l1)),
                box Term::Lit(l2),
            ) => {
                if let Some(lit) = eval_bin_op(op, l1, l2) {
                    *self = Term::Lit(lit);
                    true
                } else {
                    false
                }
            }
            Term::App(box Term::Abs(abs), box v2) if v2.is_value() => match abs {
                Abstraction::Lambda(body) => {
                    v2.shift(true, 0);
                    body.replace(0, v2);
                    body.shift(false, 0);
                    *self = *body.clone();
                    true
                }
                Abstraction::Unary(op) => match (op, v2) {
                    (UnOp::Minus, Term::Lit(Literal::Number(n))) => {
                        *self = Term::Lit((-*n).into());
                        true
                    }
                    (UnOp::Not, Term::Lit(Literal::True)) => {
                        *self = Term::Lit(Literal::False);
                        true
                    }
                    (UnOp::Not, Term::Lit(Literal::False)) => {
                        *self = Term::Lit(Literal::True);
                        true
                    }
                    _ => false,
                },

                _ => false,
            },
            Term::App(v1, t2) if v1.is_value() => t2.step(),
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
            _ => false,
        }
    }

    pub fn evaluate(&mut self) {
        while self.step() {}
    }
}

fn eval_bin_op(op: &BinOp, l1: &Literal, l2: &Literal) -> Option<Literal> {
    use Literal::*;
    match (op, l1, l2) {
        (BinOp::Plus, Number(n1), Number(n2)) => Some((n1 + n2).into()),
        (BinOp::Minus, Number(n1), Number(n2)) => Some((n1 - n2).into()),
        (BinOp::Times, Number(n1), Number(n2)) => Some((n1 * n2).into()),
        (BinOp::Divide, Number(n1), Number(n2)) => Some((n1 / n2).into()),
        (BinOp::Modulo, Number(n1), Number(n2)) => Some((n1 % n2).into()),
        (BinOp::LessThan, Number(n1), Number(n2)) => Some((n1 < n2).into()),
        (BinOp::LessThanOrEqual, Number(n1), Number(n2)) => Some((n1 <= n2).into()),
        (BinOp::GreaterThan, Number(n1), Number(n2)) => Some((n1 > n2).into()),
        (BinOp::GreaterThanOrEqual, Number(n1), Number(n2)) => Some((n1 >= n2).into()),
        (BinOp::Equal, _, _) => Some((l1 == l2).into()),
        (BinOp::NotEqual, _, _) => Some((l1 != l2).into()),
        (BinOp::And, True, True) => Some(True),
        (BinOp::And, True, False) => Some(False),
        (BinOp::And, False, True) => Some(False),
        (BinOp::And, False, False) => Some(False),
        (BinOp::Or, True, True) => Some(True),
        (BinOp::Or, True, False) => Some(True),
        (BinOp::Or, False, True) => Some(True),
        (BinOp::Or, False, False) => Some(False),
        _ => None,
    }
}
