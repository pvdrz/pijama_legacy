//! The Pijama type-checker.
//!
//! This module contains all the functions and types required to do type checking over the MIR of a
//! program.
use pijama_ast::{
    ty::{Binding, Ty, TyError, TyResult},
    BinOp, Literal, Located, Location, Name, Primitive, UnOp,
};

use crate::mir::{LetKind, Term};

/// Function that type-checks a term and returns its type.
///
/// This function must always be called in the "root" term of the program. Otherwise, the type
/// checker might not have all the bindings required to do its job.
pub fn ty_check(term: &Located<Term<'_>>) -> TyResult<Located<Ty>> {
    Context::default().type_of(&term)
}

/// Function that returns an unexpected type error if the types passed to it are not equal.
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
///
/// This only uses references to its parameters instead of using them by value.
macro_rules! ensure_ty {
    ($expected:expr, $found:expr) => {
        expect_ty(&$expected, &$found)
    };
    ($expected:expr, $found:expr, $( $other:expr ),*) => {
        expect_ty(&$expected, &$found)$(.and_then(|_| expect_ty(&$expected, &$other)))*
    };
}

/// A typing context.
///
/// This structure traverses the MIR of a program and checks the well-typedness of its inner terms.
/// A context can only have the variables that have been bound in the scope of the term is typing.
#[derive(Default)]
struct Context<'a> {
    /// Stack for the type bindings done in the current scope.
    ///
    /// Ever time a new binding is done via an abstraction or let binding term it is required to push
    /// that binding into this stack, and pop it after traversing the term.
    inner: Vec<Binding<'a>>,
}

impl<'a> Context<'a> {
    /// Returns the type of a term.
    ///
    /// The location of the type returned by this function is such that showing a type error
    /// actually points to the term causing the error. Most of the time this is the same location
    /// as the one of the term that's being typed.
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
            Term::Let(kind, name, t1, t2) => {
                self.type_of_let(loc, kind, name, t1.as_ref(), t2.as_ref())
            }
            Term::Cond(t1, t2, t3) => self.type_of_cond(loc, t1.as_ref(), t2.as_ref(), t3.as_ref()),
            Term::Seq(t1, t2) => self.type_of_seq(loc, t1.as_ref(), t2.as_ref()),
            Term::PrimFn(prim) => unreachable!(
                "Primitives always need special case handling but got {:?}",
                prim
            ),
        }
    }

    /// Returns the type of a literal.
    ///
    /// The type of a literal is only decided by its variant so this cannot fail.
    fn type_of_lit(&mut self, loc: Location, lit: &Literal) -> TyResult<Located<Ty>> {
        let ty = match lit {
            Literal::Unit => Ty::Unit,
            Literal::Bool(_) => Ty::Bool,
            Literal::Number(_) => Ty::Int,
        };
        Ok(Located::new(ty, loc))
    }

    /// Returns the type of a variable.
    ///
    /// To type a variable, it must have been binded beforehand using a let binding or an
    /// abstraction and added to the context. If the variable is not in the current context, this
    /// method returns an error stating that the variable is unbounded.
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

    /// Returns the type of an abstraction.
    ///
    /// To type an abstraction, we need to add the binding done by the abstraction to the current
    /// context and then type its body. If the body can be typed succesfully, the type of the
    /// abstraction is `T` -> `U` where `T` is the type of the binding and `U` the type of the
    /// body.
    ///
    /// Afterwards we need to remove the binding from the context because that binding is only
    /// valid inside the body of the function (lexical scoping). This function panics if it's not
    /// possible to remove the last added binding to the context (which should be the one this
    /// method added before).
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

    /// Returns the type of an unary operation.
    ///
    /// The type of an unary operation depends on its operator:
    /// - If it is a negation, the operand must have type `Int`.
    /// - If it is a logical not, the operand must have type `Bool`.
    ///
    /// If that check succeeds, the operation has the same type as the operand.
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

    /// Returns the type of an binary operation.
    ///
    /// The type of a binary operation depends on its operator:
    /// - If it is an arithmetic operator, the operands must have type `Int`.
    /// - If it is a logic operator, the operands must have type `Bool`.
    /// - If it is `Eq` or `Neq`, the operands must have the same type.
    /// - If it is any other comparison operator, the operands must have type `Bool`.
    ///
    /// If that check succeeds, the operation has type `Bool` unless it is an arithmetic operation,
    /// which has type `Int`.
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
    /// Returns the type of an application.
    ///
    /// If the first term of the application is a primitive function, the typing is delegated to
    /// another method.
    ///
    /// Otherwise, the first term must have type `T -> U` and the second term must have type `U`.
    /// If that's the case, the type of the application is `U`. If that's not the case an error is
    /// returned, either because the first term is not a function, or because the return type of
    /// the first term doesn't match the type of the second term.
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

    /// Returns the type of a let binding.
    ///
    /// Typing a let binding requires adding a type binding for the name in the context. The name
    /// is binded to whatever type has the first term. Then, the type of the let binding is
    /// the same as the type of the second term.
    ///
    /// If the user provided a type anotation, the inferred type for the first name must coincide
    /// with such anotation, otherwise an error is returned.
    ///
    /// If the let binding is recursive. A type binding with the name and the type provided by the
    /// annotation is added to the context before inferring any type in order to guarantee that the
    /// name of the let binding will be in scope.
    ///
    /// Like when typing abstractions, the type binding added to the context must be removed to
    /// avoid leaking the binding to the outer scopes. This function returns an error if it is not
    /// possible to remove such binding.
    fn type_of_let(
        &mut self,
        loc: Location,
        kind: &LetKind,
        name: &Located<Name<'a>>,
        t1: &Located<Term<'a>>,
        t2: &Located<Term<'a>>,
    ) -> TyResult<Located<Ty>> {
        match kind {
            LetKind::NonRec(opt_ty) => {
                let ty1 = self.type_of(t1)?;

                if let Some(ty) = opt_ty {
                    ensure_ty!(ty.content, ty1)?;
                }

                self.inner.push(Binding {
                    name: name.content,
                    ty: ty1.content,
                });
            }
            LetKind::Rec(ty) => {
                self.inner.push(Binding {
                    name: name.content,
                    ty: ty.content.clone(),
                });

                self.type_of(t1)?;
            }
        };

        let ty2 = self.type_of(t2)?;
        self.inner.pop().unwrap();
        Ok(Located::new(ty2.content, loc))
    }

    /// Returns the type of a conditional.
    ///
    /// Typing a conditional requires that the condition has type `Bool` and that both branches
    /// have the same type. If that's the case, the conditional has the same type as the branches.
    /// Otherwise an error is returned indicating which requirement was not satisfied.
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

    /// Returns the type of a sequence.
    ///
    /// Typing a sequence requires that the first term has type `Unit`. This is because terms
    /// cannot be simply ommited during evaluation (this is a limitation of the LIR). If that's the
    /// case the type of the sequence is the same as the type of the second term. Otherwise an
    /// error is returned indicating that the first term is not an `Unit`.
    fn type_of_seq(
        &mut self,
        _loc: Location,
        t1: &Located<Term<'a>>,
        t2: &Located<Term<'a>>,
    ) -> TyResult<Located<Ty>> {
        let ty1 = self.type_of(t1)?;
        ensure_ty!(Ty::Unit, ty1)?;
        // FIXME: this is the only method that doesn't use the location of the Term to reflect its
        // own location. If we can this, all the `type_of_*` methods coud return `TyResult<Ty>`
        self.type_of(t2)
    }

    /// Returns the type of an application of a primitive function.
    ///
    /// The type depends on which primitive function is being applied:
    ///
    /// - If the primitive is `print`, the type of the application is `Unit`.
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
