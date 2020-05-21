use thiserror::Error;

use crate::{
    ast::{BinOp, Literal, Located, Location, UnOp},
    mir::Term,
    ty::{Binding, Ty},
};

pub fn ty_check(term: &Located<Term<'_>>) -> TyResult<Located<Ty>> {
    Context::default().type_of(&term)
}

pub fn expect_ty(expected: Ty, found: Located<Ty>) -> TyResult {
    if expected == found.content {
        Ok(expected)
    } else {
        Err(TyError::Unexpected { expected, found })
    }
}

/// Macro version of `expect_ty` that accepts a comma separated list of types to check.
macro_rules! ensure_ty {
    ($expected:expr, $found:expr) => {
        crate::ty::expect_ty($expected, $found)
    };
    ($expected:expr, $found:expr, $( $other:expr ),*) => {
        crate::ty::expect_ty($expected, $found)$(.and_then(|_| crate::ty::expect_ty($expected, $other)))*
    };
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum TyError {
    #[error("Unexpected type: expected {expected}, found {found}")]
    Unexpected { expected: Ty, found: Located<Ty> },
    #[error("Name {0} is not bounded")]
    Unbound(Located<String>),
    #[error("Unexpected type: expected function, found {0}")]
    ExpectedFn(Located<Ty>),
    #[error("Unexpected type: expected a basic type, found {0}")]
    ExpectedBasic(Located<Ty>),
    #[error("Missing type: type cannot be inferred")]
    Missing(Located<()>),
}

impl TyError {
    pub fn loc(&self) -> Location {
        match self {
            TyError::Unexpected { found, .. } => found.loc,
            TyError::Unbound(name) => name.loc,
            TyError::ExpectedBasic(ty) | TyError::ExpectedFn(ty) => ty.loc,
            TyError::Missing(unit) => unit.loc,
        }
    }
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
            Term::Lit(lit) => match lit {
                Literal::Unit => Ty::Unit,
                Literal::Bool(_) => Ty::Bool,
                Literal::Number(_) => Ty::Int,
            },
            Term::Var(name) => self
                .inner
                .iter()
                .find(|bind| bind.name == *name)
                .ok_or_else(|| TyError::Unbound(Located::new(name.0.to_string(), loc)))?
                .ty
                .clone(),
            Term::Abs(bind, body) => {
                self.inner.push(bind.clone());
                let ty = self.type_of(body.as_ref())?.content;
                self.inner.pop().unwrap();
                Ty::Arrow(Box::new(bind.ty.clone()), Box::new(ty))
            }
            Term::UnaryOp(op, term) => {
                let ty = self.type_of(term.as_ref())?;
                match op {
                    UnOp::Neg => ensure_ty!(Ty::Int, ty)?,
                    UnOp::Not => ensure_ty!(Ty::Bool, ty)?,
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
                        Ty::Bool
                    }
                    BinOp::Eq | BinOp::Neq => {
                        ensure_ty!(ty1.content, ty2)?;
                        Ty::Bool
                    }
                }
            }
            Term::App(t1, t2) => {
                let ty1 = self.type_of(t1.as_ref())?;
                let ty2 = self.type_of(t2.as_ref())?;
                match ty1.content {
                    Ty::Arrow(ty11, ty) => {
                        ensure_ty!(*ty11, ty2)?;
                        *ty
                    }
                    _ => return Err(TyError::ExpectedFn(ty1)),
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
                ty2.content
            }
            Term::Cond(t1, t2, t3) => {
                let ty1 = self.type_of(t1.as_ref())?;
                let ty2 = self.type_of(t2.as_ref())?;
                let ty3 = self.type_of(t3.as_ref())?;
                ensure_ty!(Ty::Bool, ty1)?;
                ensure_ty!(ty2.content, ty3)?
            }
            Term::Seq(t1, t2) => {
                let ty1 = self.type_of(t1.as_ref())?;
                ensure_ty!(Ty::Unit, ty1)?;
                return self.type_of(t2.as_ref());
            }
            Term::Fix(t1) => {
                let ty = self.type_of(t1.as_ref())?;
                if let Ty::Arrow(ty1, ty2) = ty.content {
                    ensure_ty!(*ty1, Located::new(*ty2, ty.loc))?
                } else {
                    return Err(TyError::ExpectedFn(ty));
                }
            }
            Term::PrimFn(prim) => match prim {
                _ => panic!("There aren't any primitives yet!"),
            },
        };
        Ok(Located::new(ty, loc))
    }
}
