use thiserror::Error;

use crate::{
    ast::{BinOp, Literal, Located, UnOp, Location},
    mir::Term,
    ty::{Binding, Ty},
};

pub fn ty_check(term: &Located<Term<'_>>) -> TyResult<Located<Ty>> {
    Context::default().type_of(&term)
}

pub fn expect_ty(expected: Ty, found: Located<Ty>) -> TyResult<Located<Ty>> {
    if expected == found.content {
        Ok(found)
    } else {
        Err(TyError {
            loc: found.loc,
            kind: TyErrorKind::Mismatch {
                expected,
                found: found.content,
            },
        })
    }
}

/// Macro version of `expect_ty` that accepts a comma separated list of types to check.
macro_rules! ensure_ty {
    ($expected:path, $found:ident, $( $other:ident ),*) => {
        // Using a closure to benefit from early exit via ? without interfering with the caller
        move || -> TyResult<Located<Ty>> {
            let ty = expect_ty($expected, $found)?;
            $(
                let ty = expect_ty($expected, $other)?;
            )*
            Ok(ty)
        }()
    }
}

#[derive(Error, Debug)]
#[error("{kind}")]
pub struct TyError {
    pub loc: Location,
    pub kind: TyErrorKind,
}

#[derive(Error, Debug)]
pub enum TyErrorKind {
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
    fn type_of(&mut self, term: &Located<Term<'a>>) -> TyResult<Located<Ty>> {
        let loc = term.loc;
        let ty = match &term.content {
            Term::Lit(lit) => {
                let ty = match lit {
                    Literal::Unit => Ty::Unit,
                    Literal::Bool(_) => Ty::Bool,
                    Literal::Number(_) => Ty::Int,
                };
                Located::new(ty, loc)
            }
            Term::Var(name) => {
                let ty = self
                    .inner
                    .iter()
                    .find(|bind| bind.name == *name)
                    .ok_or_else(|| TyError {
                        loc,
                        kind: TyErrorKind::Unbound(name.0.to_string()),
                    })?
                    .ty
                    .clone();
                Located::new(ty, loc)
            }
            Term::Abs(bind, body) => {
                self.inner.push(bind.clone());
                let ty = self.type_of(body.as_ref())?.content;
                self.inner.pop().unwrap();
                Located::new(Ty::Arrow(Box::new(bind.ty.clone()), Box::new(ty)), loc)
            }
            Term::UnaryOp(op, term) => {
                let ty = self.type_of(term.as_ref())?;
                match op {
                    UnOp::Neg => expect_ty(Ty::Int, ty)?,
                    UnOp::Not => expect_ty(Ty::Bool, ty)?,
                }
            }
            Term::BinaryOp(op, t1, t2) => {
                let ty1 = self.type_of(t1.as_ref())?;
                let ty2 = self.type_of(t2.as_ref())?;
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
                        Located::new(Ty::Bool, loc)
                    }
                    BinOp::Eq | BinOp::Neq => {
                        expect_ty(ty1.content, ty2)?;
                        Located::new(Ty::Bool, loc)
                    }
                }
            }
            Term::App(t1, t2) => {
                let ty1 = self.type_of(t1.as_ref())?;
                let ty2 = self.type_of(t2.as_ref())?;
                match ty1.content {
                    Ty::Arrow(ty11, ty) => {
                        expect_ty(*ty11, ty2)?;
                        Located::new(*ty, loc)
                    }
                    _ => {
                        return Err(TyError {
                            kind: TyErrorKind::NotFn(ty1.content),
                            loc: ty1.loc,
                        })
                    }
                }
            }
            Term::Let(name, t1, t2) => {
                let bind = self.type_of(t1.as_ref()).map(|ty| Binding {
                    name: name.content,
                    ty: ty.content,
                })?;
                self.inner.push(bind);
                let ty2 = self.type_of(t2.as_ref())?;
                self.inner.pop().unwrap();
                ty2
            }
            Term::Cond(t1, t2, t3) => {
                let ty1 = self.type_of(t1.as_ref())?;
                let ty2 = self.type_of(t2.as_ref())?;
                let ty3 = self.type_of(t3.as_ref())?;
                expect_ty(Ty::Bool, ty1)?;
                expect_ty(ty2.content, ty3)?
            }
            Term::Seq(t1, t2) => {
                let ty1 = self.type_of(t1.as_ref())?;
                expect_ty(Ty::Unit, ty1)?;
                self.type_of(t2.as_ref())?
            }
            Term::Fix(t1) => {
                let ty = self.type_of(t1.as_ref())?;
                match ty.content {
                    Ty::Arrow(box ty1, box ty2) => expect_ty(ty1, Located::new(ty2, ty.loc))?,
                    _ => {
                        return Err(TyError {
                            kind: TyErrorKind::NotFn(ty.content),
                            loc: ty.loc,
                        });
                    }
                }
            }
        };
        Ok(ty)
    }
}
