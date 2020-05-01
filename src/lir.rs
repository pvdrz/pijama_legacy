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
    fn is_literal(&self) -> bool {
        match self {
            Term::Lit(_) => true,
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

    fn reduce(&mut self) -> bool {
        match self {
            Term::App(t1, t2) => match t1.as_mut() {
                Term::Abs(abs) => match abs {
                    Abstraction::Lambda(body) => {
                        t2.shift(true, 0);
                        body.replace(0, t2);
                        body.shift(false, 0);
                        *self = *body.clone();
                        true
                    }
                    Abstraction::Unary(op) => {
                        let lit = match (op, t2.as_ref()) {
                            (UnOp::Minus, Term::Lit(Literal::Number(num))) => Literal::Number(-num),
                            (UnOp::Not, Term::Lit(Literal::True)) => Literal::False,
                            (UnOp::Not, Term::Lit(Literal::False)) => Literal::True,
                            _ => return t1.reduce() || t2.reduce(),
                        };
                        *self = Term::Lit(lit);
                        true
                    }
                    _ => false,
                },
                Term::App(t11, t12) => match t11.as_ref() {
                    Term::Abs(Abstraction::Binary(op)) => {
                        let lit = match (t12.as_ref(), t2.as_ref()) {
                            (Term::Lit(l1), Term::Lit(l2)) => match (op, l1, l2) {
                                (BinOp::Plus, Literal::Number(n1), Literal::Number(n2)) => {
                                    (n1 + n2).into()
                                }
                                (BinOp::Minus, Literal::Number(n1), Literal::Number(n2)) => {
                                    (n1 - n2).into()
                                }
                                (BinOp::Times, Literal::Number(n1), Literal::Number(n2)) => {
                                    (n1 * n2).into()
                                }
                                (BinOp::Divide, Literal::Number(n1), Literal::Number(n2)) => {
                                    (n1 / n2).into()
                                }
                                (BinOp::Modulo, Literal::Number(n1), Literal::Number(n2)) => {
                                    (n1 % n2).into()
                                }
                                (BinOp::LessThan, Literal::Number(n1), Literal::Number(n2)) => {
                                    (n1 < n2).into()
                                }
                                (BinOp::GreaterThan, Literal::Number(n1), Literal::Number(n2)) => {
                                    (n1 > n2).into()
                                }
                                (
                                    BinOp::LessThanOrEqual,
                                    Literal::Number(n1),
                                    Literal::Number(n2),
                                ) => (n1 <= n2).into(),
                                (
                                    BinOp::GreaterThanOrEqual,
                                    Literal::Number(n1),
                                    Literal::Number(n2),
                                ) => (n1 >= n2).into(),
                                (BinOp::Equal, l1, l2) => (l1 == l2).into(),
                                (BinOp::NotEqual, l1, l2) => (l1 != l2).into(),
                                _ => return t1.reduce() || t2.reduce(),
                            },
                            _ => return t1.reduce() || t2.reduce(),
                        };

                        *self = Term::Lit(lit);
                        true
                    }
                    _ => t1.reduce() || t2.reduce(),
                },
                _ => t1.reduce() || t2.reduce(),
            },
            Term::Cond(t1, t2, t3) => {
                t2.reduce()
                    || t3.reduce()
                    || match t1.as_mut() {
                        Term::Lit(Literal::True) => {
                            *self = *t2.clone();
                            true
                        }
                        Term::Lit(Literal::False) => {
                            *self = *t3.clone();
                            true
                        }
                        _ => t1.reduce(),
                    }
            }
            _ => false,
        }
    }

    pub fn evaluate(&mut self) {
        while self.reduce() {}
    }
}
