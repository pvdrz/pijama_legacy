use crate::{
    ast::{BinOp, Literal, Located, Location, Name, Primitive, UnOp},
    mir::Term,
    ty::{Binding, Ty, TyError, TyResult},
};

pub fn ty_check(term: &Located<Term<'_>>) -> TyResult<Located<Ty>> {
    Context::default().type_of(&term)
}

pub fn expect_ty(expected: &Ty, found: &Located<Ty>) -> TyResult<()> {
    if *expected == found.content {
        Ok(())
    } else {
        Err(TyError::Unexpected {
            expected: expected.clone(),
            found: found.clone(),
        })
    }
}

/// Macro version of `expect_ty` that accepts a comma separated list of types to check.
macro_rules! ensure_ty {
    ($expected:expr, $found:expr) => {
        crate::ty::expect_ty(&$expected, &$found)
    };
    ($expected:expr, $found:expr, $( $other:expr ),*) => {
        crate::ty::expect_ty(&$expected, &$found)$(.and_then(|_| crate::ty::expect_ty(&$expected, &$other)))*
    };
}

#[derive(Default)]
struct Context<'a> {
    inner: Vec<Binding<'a>>,
}

impl<'a> Context<'a> {
    fn type_of(&mut self, term: &Located<Term<'a>>) -> TyResult<Located<Ty>> {
        let loc = term.loc;
        match &term.content {
            Term::Lit(lit) => self.type_of_lit(loc, lit),
            Term::Var(name) => self.type_of_var(loc, name),
            Term::Abs(bind, body) => self.type_of_abs(loc, bind, body.as_ref()),
            Term::UnaryOp(op, term) => self.type_of_unary_op(loc, *op, term.as_ref()),
            Term::BinaryOp(op, t1, t2) => {
                self.type_of_binary_op(loc, *op, t1.as_ref(), t2.as_ref())
            }
            Term::App(t1, t2) => self.type_of_app(loc, t1.as_ref(), t2.as_ref()),
            Term::Let(name, t1, t2) => self.type_of_let(loc, name, t1.as_ref(), t2.as_ref()),
            Term::Cond(t1, t2, t3) => self.type_of_cond(loc, t1.as_ref(), t2.as_ref(), t3.as_ref()),
            Term::Seq(t1, t2) => self.type_of_seq(loc, t1.as_ref(), t2.as_ref()),
            Term::Fix(t1) => self.type_of_fix(loc, t1.as_ref()),
            Term::PrimFn(prim) => unreachable!(
                "Primitives always need special case handling but got {:?}",
                prim
            ),
        }
    }

    fn type_of_lit(&mut self, loc: Location, lit: &Literal) -> TyResult<Located<Ty>> {
        let ty = match lit {
            Literal::Unit => Ty::Unit,
            Literal::Bool(_) => Ty::Bool,
            Literal::Number(_) => Ty::Int,
        };
        Ok(Located::new(ty, loc))
    }

    fn type_of_var(&mut self, loc: Location, name: &Name<'a>) -> TyResult<Located<Ty>> {
        let ty = self
            .inner
            .iter()
            .find(|bind| bind.name == *name)
            .ok_or_else(|| TyError::Unbound(Located::new(name.0.to_string(), loc)))?
            .ty
            .clone();
        Ok(Located::new(ty, loc))
    }

    fn type_of_abs(
        &mut self,
        loc: Location,
        bind: &Binding<'a>,
        body: &Located<Term<'a>>,
    ) -> TyResult<Located<Ty>> {
        self.inner.push(bind.clone());
        let ty = self.type_of(body)?.content;
        self.inner.pop().unwrap();
        let ty = Ty::Arrow(Box::new(bind.ty.clone()), Box::new(ty));
        Ok(Located::new(ty, loc))
    }

    fn type_of_unary_op(
        &mut self,
        loc: Location,
        op: UnOp,
        term: &Located<Term<'a>>,
    ) -> TyResult<Located<Ty>> {
        let ty = self.type_of(term)?;
        match op {
            UnOp::Neg => ensure_ty!(Ty::Int, ty)?,
            UnOp::Not => ensure_ty!(Ty::Bool, ty)?,
        };
        Ok(Located::new(ty.content, loc))
    }

    fn type_of_binary_op(
        &mut self,
        loc: Location,
        op: BinOp,
        t1: &Located<Term<'a>>,
        t2: &Located<Term<'a>>,
    ) -> TyResult<Located<Ty>> {
        let ty1 = self.type_of(t1)?;
        let ty2 = self.type_of(t2)?;
        let ty = match op {
            BinOp::Add
            | BinOp::Sub
            | BinOp::Mul
            | BinOp::Div
            | BinOp::Rem
            | BinOp::BitAnd
            | BinOp::BitOr
            | BinOp::BitXor
            | BinOp::Shr
            | BinOp::Shl => {
                ensure_ty!(Ty::Int, ty1, ty2)?;
                Ty::Int
            }
            BinOp::Or | BinOp::And => {
                ensure_ty!(Ty::Bool, ty1, ty2)?;
                Ty::Bool
            }
            BinOp::Lt | BinOp::Gt | BinOp::Lte | BinOp::Gte => {
                ensure_ty!(Ty::Int, ty1, ty2)?;
                Ty::Bool
            }
            BinOp::Eq | BinOp::Neq => {
                ensure_ty!(ty1.content, ty2)?;
                Ty::Bool
            }
        };
        Ok(Located::new(ty, loc))
    }

    fn type_of_app(
        &mut self,
        loc: Location,
        t1: &Located<Term<'a>>,
        t2: &Located<Term<'a>>,
    ) -> TyResult<Located<Ty>> {
        if let Term::PrimFn(primitive) = t1.content {
            self.type_of_prim_app(loc, primitive, t2)
        } else {
            let ty1 = self.type_of(t1)?;
            let ty2 = self.type_of(t2)?;
            match ty1.content {
                Ty::Arrow(ty11, ty) => {
                    ensure_ty!(ty11, ty2)?;
                    Ok(Located::new(*ty, loc))
                }
                _ => Err(TyError::ExpectedFn(ty1)),
            }
        }
    }

    fn type_of_let(
        &mut self,
        loc: Location,
        name: &Located<Name<'a>>,
        t1: &Located<Term<'a>>,
        t2: &Located<Term<'a>>,
    ) -> TyResult<Located<Ty>> {
        let bind = self.type_of(t1).map(|ty| Binding {
            name: name.content,
            ty: ty.content,
        })?;
        self.inner.push(bind);
        let ty2 = self.type_of(t2)?;
        self.inner.pop().unwrap();
        Ok(Located::new(ty2.content, loc))
    }

    fn type_of_cond(
        &mut self,
        loc: Location,
        t1: &Located<Term<'a>>,
        t2: &Located<Term<'a>>,
        t3: &Located<Term<'a>>,
    ) -> TyResult<Located<Ty>> {
        let ty1 = self.type_of(t1)?;
        let ty2 = self.type_of(t2)?;
        let ty3 = self.type_of(t3)?;
        ensure_ty!(Ty::Bool, ty1)?;
        ensure_ty!(ty2.content, ty3)?;
        Ok(Located::new(ty2.content, loc))
    }

    fn type_of_seq(
        &mut self,
        _loc: Location,
        t1: &Located<Term<'a>>,
        t2: &Located<Term<'a>>,
    ) -> TyResult<Located<Ty>> {
        let ty1 = self.type_of(t1)?;
        ensure_ty!(Ty::Unit, ty1)?;
        self.type_of(t2)
    }

    fn type_of_fix(&mut self, loc: Location, t1: &Located<Term<'a>>) -> TyResult<Located<Ty>> {
        let ty = self.type_of(t1)?;
        if let Ty::Arrow(ty1, ty2) = ty.content {
            ensure_ty!(*ty1, Located::new(*ty2, ty.loc))?;
            Ok(Located::new(*ty1, loc))
        } else {
            Err(TyError::ExpectedFn(ty))
        }
    }

    fn type_of_prim_app(
        &mut self,
        loc: Location,
        prim: Primitive,
        _arg: &Located<Term>,
    ) -> TyResult<Located<Ty>> {
        match prim {
            Primitive::Print => Ok(Located::new(Ty::Unit, loc)),
        }
    }
}
