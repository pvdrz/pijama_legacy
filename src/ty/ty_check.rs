use thiserror::Error;

use crate::{
    ast::{BinOp, Literal, UnOp},
    mir::Term,
    ty::{Binding, Ty},
};

pub fn ty_check(term: &Term<'_>) -> TyResult<Ty> {
    Context::default().type_of(term)
}

pub fn expect_ty(expected: Ty, found: Ty) -> TyResult<Ty> {
    if expected == found {
        Ok(expected)
    } else {
        Err(TyError::Mismatch { expected, found })
    }
}

/// Macro version of `expect_ty` that accepts a comma separated list of types to check.
macro_rules! ensure_ty {
    ($expected:path, $found:ident, $( $other:ident ),*) => {
        // Using a closure to benefit from early exit via ? without interfering with the caller
        move || -> TyResult<Ty> {
            let ty = expect_ty($expected, $found)?;
            $(
                let ty = expect_ty(ty, $other)?;
            )*
            Ok(ty)
        }()
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
                .ok_or_else(|| TyError::Unbound(name.0.to_string()))?
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
                    UnOp::Neg => expect_ty(Ty::Int, ty)?,
                    UnOp::Not => expect_ty(Ty::Bool, ty)?,
                }
            }
            Term::BinaryOp(op, t1, t2) => {
                let ty1 = self.type_of(t1)?;
                let ty2 = self.type_of(t2)?;
                match op {
                    BinOp::Add
                    | BinOp::Sub
                    | BinOp::Mul
                    | BinOp::Div
                    | BinOp::Rem
                    | BinOp::BitAnd
                    | BinOp::BitOr
                    | BinOp::BitXor
                    | BinOp::Shr
                    | BinOp::Shl => ensure_ty!(Ty::Int, ty1, ty2)?,
                    BinOp::Or | BinOp::And => ensure_ty!(Ty::Bool, ty1, ty2)?,
                    BinOp::Lt | BinOp::Gt | BinOp::Lte | BinOp::Gte => {
                        ensure_ty!(Ty::Int, ty1, ty2)?;
                        Ty::Bool
                    }
                    BinOp::Eq | BinOp::Neq => {
                        expect_ty(ty1, ty2)?;
                        Ty::Bool
                    }
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
