use thiserror::Error;

use crate::ast::{BinOp, Literal, UnOp};
use crate::mir::{Abstraction, Term};
use crate::ty::{Binding, Ty};
use crate::LangResult;

pub fn ty_check(term: &Term<'_>) -> LangResult<Ty> {
    Context::default().type_of(term).map_err(Into::into)
}

#[derive(Error, Debug)]
pub enum TyError {
    #[error("Type mismatch: expected {expected}, found {found}")]
    Mismatch { expected: Ty, found: Ty },
    #[error("Name {0} is unbounded")]
    Unbound(String),
    #[error("Type mismatch: expected function, found {0}")]
    NotFn(Ty),
}

type TyResult<T = Ty> = Result<T, TyError>;

#[derive(Default)]
struct Context<'a> {
    inner: Vec<Binding<'a>>,
}

impl<'a> Context<'a> {
    fn type_of(&mut self, term: &Term<'a>) -> TyResult {
        let ty = match term {
            Term::Lit(lit) => match lit {
                Literal::Unit => Ty::Unit,
                Literal::True | Literal::False => Ty::Bool,
                Literal::Number(_) => Ty::Int,
            },
            Term::Var(name) => self
                .inner
                .iter()
                .find(|bind| bind.name == *name)
                .ok_or_else(|| TyError::Unbound(name.0.to_owned()))?
                .ty
                .clone(),
            Term::Abs(abs) => match abs {
                Abstraction::Lambda(bind, body) => {
                    self.inner.push(bind.clone());
                    let ty = self.type_of(body)?;
                    self.inner.pop().unwrap();
                    Ty::Arrow(Box::new(bind.ty.clone()), Box::new(ty))
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
                    | BinOp::GreaterThanOrEqual => Ty::Arrow(
                        Box::new(Ty::Int),
                        Box::new(Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Bool))),
                    ),
                    BinOp::Equal | BinOp::NotEqual => todo!(),
                },
            },
            Term::App(t1, t2) => {
                let ty1 = self.type_of(t1)?;
                let ty2 = self.type_of(t2)?;
                match ty1 {
                    Ty::Arrow(ty11, ty) => {
                        if *ty11 == ty2 {
                            *ty
                        } else {
                            return Err(TyError::Mismatch {
                                expected: *ty11,
                                found: ty2,
                            });
                        }
                    }
                    _ => return Err(TyError::NotFn(ty1)),
                }
            }
            &Term::Let(name, ref t1, ref t2) => {
                let bind = self.type_of(t1).map(|ty| Binding { name, ty })?;
                self.inner.push(bind);
                let ty2 = self.type_of(t2)?;
                self.inner.pop().unwrap();
                ty2
            }
            Term::Cond(t1, t2, t3) => {
                let ty1 = self.type_of(t1)?;
                let ty2 = self.type_of(t2)?;
                let ty3 = self.type_of(t3)?;
                if ty1 == Ty::Bool {
                    if ty2 == ty3 {
                        ty3
                    } else {
                        return Err(TyError::Mismatch {
                            expected: ty2,
                            found: ty3,
                        });
                    }
                } else {
                    return Err(TyError::Mismatch {
                        expected: Ty::Bool,
                        found: ty1,
                    });
                }
            }
            Term::Seq(t1, t2) => {
                let ty1 = self.type_of(t1)?;
                if ty1 == Ty::Unit {
                    self.type_of(t2)?
                } else {
                    return Err(TyError::Mismatch {
                        expected: Ty::Unit,
                        found: ty1,
                    });
                }
            }
        };
        Ok(ty)
    }
}
