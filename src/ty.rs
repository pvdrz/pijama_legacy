use std::fmt;

use crate::ast::{BinOp, Literal, Name, UnOp};
use crate::mir::{Abstraction, Term};

pub fn ty_check(term: &Term) -> Ty {
    Context::default().type_of(term)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    Bool,
    Int,
    Unit,
    Arrow(Box<Ty>, Box<Ty>),
}

impl<'a> fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Ty::*;
        match self {
            Bool => write!(f, "Bool"),
            Int => write!(f, "Int"),
            Unit => write!(f, "Unit"),
            Arrow(t1, t2) => write!(f, "({} -> {})", t1, t2),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Binding<'a> {
    pub name: Name<'a>,
    pub ty: Ty,
}

#[derive(Default)]
struct Context<'a> {
    inner: Vec<Binding<'a>>,
}

impl<'a> Context<'a> {
    fn type_of(&mut self, term: &Term<'a>) -> Ty {
        match term {
            Term::Lit(lit) => match lit {
                Literal::Unit => Ty::Unit,
                Literal::True | Literal::False => Ty::Bool,
                Literal::Number(_) => Ty::Int,
            },
            Term::Var(name) => self
                .inner
                .iter()
                .find(|bind| bind.name == *name)
                .unwrap()
                .ty
                .clone(),

            Term::Abs(abs) => match abs {
                Abstraction::Lambda(bind, body) => {
                    self.inner.push(bind.clone());
                    let ty = self.type_of(body);
                    let bind = self.inner.pop().unwrap();
                    Ty::Arrow(Box::new(bind.ty), Box::new(ty))
                }
                Abstraction::Unary(op) => match op {
                    UnOp::Minus => Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Int)),
                    UnOp::Not => Ty::Arrow(Box::new(Ty::Bool), Box::new(Ty::Bool)),
                },
                Abstraction::Binary(op) => match op {
                    BinOp::Plus | BinOp::Minus | BinOp::Times | BinOp::Divide | BinOp::Modulo => {
                        Ty::Arrow(
                            Box::new(Ty::Int),
                            Box::new(Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Int))),
                        )
                    }
                    BinOp::Or | BinOp::And => Ty::Arrow(
                        Box::new(Ty::Bool),
                        Box::new(Ty::Arrow(Box::new(Ty::Bool), Box::new(Ty::Bool))),
                    ),
                    BinOp::LessThan
                    | BinOp::GreaterThan
                    | BinOp::LessThanOrEqual
                    | BinOp::GreaterThanOrEqual
                    | BinOp::Equal
                    | BinOp::NotEqual => Ty::Arrow(
                        Box::new(Ty::Int),
                        Box::new(Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Bool))),
                    ),
                },
            },
            Term::App(t1, t2) => {
                let ty1 = self.type_of(t1);
                let ty2 = self.type_of(t2);
                match ty1 {
                    Ty::Arrow(ty11, ty) if *ty11 == ty2 => *ty,
                    _ => todo!(),
                }
            }
            Term::Let(name, t1, t2) => {
                let ty1 = self.type_of(t1);
                self.inner.push(Binding {
                    name: name.clone(),
                    ty: ty1,
                });
                let ty2 = self.type_of(t2);
                self.inner.pop().unwrap();
                ty2
            }
            Term::Cond(t1, t2, t3) => {
                let ty1 = self.type_of(t1);
                let ty2 = self.type_of(t2);
                let ty3 = self.type_of(t3);
                if ty1 == Ty::Bool && ty2 == ty3 {
                    ty3
                } else {
                    todo!()
                }
            }
            Term::Seq(t1, t2) => {
                let ty1 = self.type_of(t1);
                if ty1 == Ty::Unit {
                    self.type_of(t2)
                } else {
                    todo!()
                }
            }
        }
    }
}
