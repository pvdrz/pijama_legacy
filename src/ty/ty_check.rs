use thiserror::Error;

use crate::ast::{BinOp, Literal, UnOp};
use crate::mir::Term;
use crate::ty::{Binding, Ty};
use crate::LangResult;

pub fn ty_check(term: &Term<'_>) -> LangResult<Ty> {
    Context::default().type_of(term).map_err(Into::into)
}

pub fn expect_ty(expected: Ty, found: Ty) -> TyResult<Ty> {
    if expected == found {
        Ok(expected)
    } else {
        Err(TyError::Mismatch { expected, found })
    }
}

#[derive(Error, Debug)]
pub enum TyError {
    #[error("Type mismatch: expected {expected}, found {found}")]
    Mismatch { expected: Ty, found: Ty },
    #[error("Name {0} is unbounded")]
    Unbound(String),
    #[error("Type mismatch: expected function, found {0}")]
    NotFn(Ty),
    #[error("Type mismatch: expected a basic type, found {0}")]
    NotBasicTy(Ty),
}

pub type TyResult<T = Ty> = Result<T, TyError>;

#[derive(Default)]
struct Context<'a> {
    inner: Vec<Binding<'a>>,
}

impl<'a> Context<'a> {
    fn type_of(&mut self, term: &Term<'a>) -> TyResult {
        let ty = match term {
            Term::Lit(lit) => match lit {
                Literal::Unit => Ty::Unit,
                Literal::Bool(_) => Ty::Bool,
                Literal::Number(_) => Ty::Int,
            },
            Term::Var(name) => self
                .inner
                .iter()
                .find(|bind| bind.name == *name)
                .ok_or_else(|| TyError::Unbound(name.0.to_owned()))?
                .ty
                .clone(),
            Term::Abs(bind, body) => {
                self.inner.push(bind.clone());
                let ty = self.type_of(body)?;
                self.inner.pop().unwrap();
                Ty::Arrow(Box::new(bind.ty.clone()), Box::new(ty))
            }
            Term::UnaryOp(op, term) => {
                let ty = self.type_of(term)?;
                match op {
                    UnOp::Minus => expect_ty(Ty::Int, ty)?,
                    UnOp::Not => expect_ty(Ty::Bool, ty)?,
                }
            }
            Term::BinaryOp(op, t1, t2) => {
                let ty1 = self.type_of(t1)?;
                let ty2 = self.type_of(t2)?;
                let ty = expect_ty(ty1, ty2)?;
                match op {
                    BinOp::Plus
                    | BinOp::Minus
                    | BinOp::Times
                    | BinOp::Divide
                    | BinOp::Modulo
                    | BinOp::BitAnd
                    | BinOp::BitOr
                    | BinOp::BitXor => expect_ty(Ty::Int, ty)?,
                    BinOp::Or | BinOp::And => expect_ty(Ty::Bool, ty)?,
                    BinOp::LessThan
                    | BinOp::GreaterThan
                    | BinOp::LessThanOrEqual
                    | BinOp::GreaterThanOrEqual => {
                        expect_ty(Ty::Int, ty)?;
                        Ty::Bool
                    }
                    BinOp::Equal | BinOp::NotEqual => Ty::Bool,
                }
            }
            Term::App(t1, t2) => {
                let ty1 = self.type_of(t1)?;
                let ty2 = self.type_of(t2)?;
                match ty1 {
                    Ty::Arrow(ty11, ty) => {
                        expect_ty(*ty11, ty2)?;
                        *ty
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
                expect_ty(Ty::Bool, ty1)?;
                expect_ty(ty2, ty3)?
            }
            Term::Seq(t1, t2) => {
                let ty1 = self.type_of(t1)?;
                expect_ty(Ty::Unit, ty1)?;
                self.type_of(t2)?
            }
            Term::Fix(t1) => match self.type_of(t1)? {
                Ty::Arrow(box ty1, box ty2) => expect_ty(ty1, ty2)?,
                ty => {
                    return Err(TyError::NotFn(ty));
                }
            },
        };
        Ok(ty)
    }
}
