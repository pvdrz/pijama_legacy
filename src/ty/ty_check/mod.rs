//! Pijama's type-checker.
//!
//! This module contains all the functions and types required to do type-checking over the MIR of a
//! program. Pijama uses constraint-based typing, which is better suited for type
//! reconstruction/inference than a regular in-place enforcement of the typing rules.
//!
//! The entry-point for this module is the `ty_check` method which does the type checking of a
//! whole program. However, most of the heavy lifting is done by the `Context` and `Unifier` types.
use pijama_ast::{BinOp, Literal, Located, Location, Name, Primitive, UnOp};

use std::collections::VecDeque;

use crate::{
    mir::{LetKind, Term},
    ty::{Ty, TyError, TyResult},
};

mod unify;

use unify::{Constraint, Unifier};

/// Function that type-checks a term and returns its type.
///
/// This function must always be called in the "root" term of the program. Otherwise, the type
/// checker might not have all the bindings required to do its job.
pub fn ty_check(term: &Located<Term<'_>>) -> TyResult<Located<Ty>> {
    // Create a new, empty context.
    let mut ctx = Context::default();
    // Obtain typing constraints and the type of `term`.
    let mut ty = ctx.type_of(&term)?;
    // Solve the constraints using unification.
    let unif = Unifier::from_ctx(ctx)?;
    // Apply the substitutions found during unification over the type of `term`.
    unif.replace(&mut ty.content);
    Ok(ty)
}

/// A type binding.
///
/// This represents the binding of a `Name` to a type and is used inside the type-checker to encode
/// that a variable has a type in the current scope.
struct TyBinding<'a> {
    name: Name<'a>,
    ty: Ty,
}

/// A typing context.
///
/// This structure traverses the MIR of a term and generates a set of constraints that must be
/// satisfied by the term to be well-typed.
///
/// A context can only have the variables that have been bound in the scope of the term is typing.
#[derive(Default)]
struct Context<'a> {
    /// Stack for the type bindings done in the current scope.
    ///
    /// Ever time a new binding is done via an abstraction or let binding term it is required to push
    /// that binding into this stack, and pop it after traversing the term.
    inner: Vec<TyBinding<'a>>,
    /// Number of created type variables.
    ///
    /// Every time a new variable is created with the `new_ty` method, this number is increased to
    /// guarantee all type variables are different.
    count: usize,
    /// Typing constraints.
    ///
    /// Each typing constraint is introduced by a particular `type_of_*` method with a suitable
    /// location in case an error needs to be returned.
    constraints: VecDeque<Located<Constraint>>,
}

impl<'a> Context<'a> {
    /// Returns a new type variable.
    ///
    /// This variable is guaranteed to be different from all the other types introduced before.
    fn new_ty(&mut self) -> Ty {
        let ty = Ty::Var(self.count);
        self.count += 1;
        ty
    }

    /// Adds a new `Constraint`.
    ///
    /// A new constraint must be added when it is required to enforce an specific typing rule.
    /// Calling this method will not enforce the rule instantly. It only stores the restriction
    /// inside the `Context` to be solved by the `Unifier` in a posterior stage. This constraint
    /// has a location that will be used as the location of the error if the constraint is
    /// impossible to satisfy.
    pub fn add_constraint(&mut self, expected: Ty, found: Ty, loc: Location) {
        let constr = Constraint::new(expected, found);
        // New constraints are front-pushed because the `Unifier` processes constraints by popping
        // them from the back. If we just back-push the constraints, we end up taking care of the
        // newer constraints first, which are more complex and can end up in less readable type
        // errors.
        self.constraints.push_front(Located::new(constr, loc))
    }

    /// Returns the type of a term.
    ///
    /// The location of the type returned by this function is such that showing a type error
    /// actually points to the term causing the error. Most of the time this is the same location
    /// as the one of the term that is being typed.
    ///
    /// Typing variables can appear in the type returned by this method (and any other type_of_*
    /// method) as unification has not taken place yet.
    fn type_of(&mut self, term: &Located<Term<'a>>) -> TyResult<Located<Ty>> {
        let loc = term.loc;
        match &term.content {
            Term::Lit(lit) => self.type_of_lit(loc, lit),
            Term::Var(name) => self.type_of_var(loc, name),
            Term::Abs(name, ty, body) => self.type_of_abs(loc, *name, ty, body.as_ref()),
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
            Term::PrimFn(prim) => self.type_of_prim_fn(loc, *prim),
        }
    }

    /// Returns the type of a literal.
    ///
    /// This rule does not add new constraints because the type of a literal is only decided by its
    /// variant.
    fn type_of_lit(&mut self, loc: Location, lit: &Literal) -> TyResult<Located<Ty>> {
        let ty = match lit {
            Literal::Unit => Ty::Unit,
            Literal::Bool(_) => Ty::Bool,
            Literal::Number(_) => Ty::Int,
        };
        Ok(loc.with_content(ty))
    }

    /// Returns the type of a variable.
    ///
    /// To type a variable, it must have been binded beforehand using a let binding or an
    /// abstraction and added to the context. If the variable is not in the current context, this
    /// method returns an error stating that the variable is unbounded.
    ///
    /// This rule does not add new constraints because the type of a variable is decided by the
    /// bindings done in the current scope.
    fn type_of_var(&mut self, loc: Location, name: &Name<'a>) -> TyResult<Located<Ty>> {
        let ty = self
            .inner
            .iter()
            .find(|bind| bind.name == *name)
            .ok_or_else(|| TyError::Unbounded(loc.with_content(name.0.to_string())))?
            .ty
            .clone();
        Ok(loc.with_content(ty))
    }

    /// Returns the type of an abstraction.
    ///
    /// To type an abstraction, we need to add the binding done by the abstraction to the current
    /// context and then type its body. If the body can be typed successfully, the type of the
    /// abstraction is `T` -> `U` where `T` is the type of the binding and `U` the type of the
    /// body.
    ///
    /// Afterwards we need to remove the binding from the context because that binding is only
    /// valid inside the body of the function (lexical scoping). This function panics if it's not
    /// possible to remove the last added binding to the context (which should be the one this
    /// method added before).
    ///
    /// This rule does not add new constraints because the type of an abstraction can be computed
    /// directly from the type of its body and argument.
    fn type_of_abs(
        &mut self,
        _loc: Location,
        name: Name<'a>,
        ty: &Ty,
        body: &Located<Term<'a>>,
    ) -> TyResult<Located<Ty>> {
        self.inner.push(TyBinding {
            name,
            ty: ty.clone(),
        });
        let ty = self.type_of(body)?;
        let bind = self.inner.pop().unwrap();

        Ok(ty.map(|ty| Ty::Arrow(Box::new(bind.ty), Box::new(ty))))
    }

    /// Returns the type of an unary operation.
    ///
    /// The type of an unary operation depends on its operator:
    /// - If it is a negation, the operand must have type `Int`.
    /// - If it is a logical not, the operand must have type `Bool`.
    ///
    /// This rule adds a constraint stating that the type of the operand must match one of the
    /// types stated above. The returned type is the same type as the operand.
    fn type_of_unary_op(
        &mut self,
        loc: Location,
        op: UnOp,
        term: &Located<Term<'a>>,
    ) -> TyResult<Located<Ty>> {
        let ty = self.type_of(term)?.content;
        let expected = match op {
            UnOp::Neg => Ty::Int,
            UnOp::Not => Ty::Bool,
        };
        self.add_constraint(expected, ty.clone(), loc);
        Ok(loc.with_content(ty))
    }

    /// Returns the type of an binary operation.
    ///
    /// The type of a binary operation depends on its operator:
    /// - If it is an arithmetic operator, the operands must have type `Int`.
    /// - If it is a logic operator, the operands must have type `Bool`.
    /// - If it is `Eq` or `Neq`, the operands must have the same type.
    /// - If it is any other comparison operator, the operands must have type `Bool`.
    ///
    /// This rule adds one of the constraints stated above. The returned type is `Bool`, unless the
    /// operation is an arithmetic operation, which has type `Int`.
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
                self.add_constraint(Ty::Int, ty1.content, ty1.loc);
                self.add_constraint(Ty::Int, ty2.content, ty2.loc);
                Ty::Int
            }
            BinOp::Or | BinOp::And => {
                self.add_constraint(Ty::Bool, ty1.content, ty1.loc);
                self.add_constraint(Ty::Bool, ty2.content, ty2.loc);
                Ty::Bool
            }
            BinOp::Lt | BinOp::Gt | BinOp::Lte | BinOp::Gte => {
                self.add_constraint(Ty::Int, ty1.content, ty1.loc);
                self.add_constraint(Ty::Int, ty2.content, ty2.loc);
                Ty::Bool
            }
            BinOp::Eq | BinOp::Neq => {
                self.add_constraint(ty1.content, ty2.content, ty2.loc);
                Ty::Bool
            }
        };
        Ok(loc.with_content(ty))
    }
    /// Returns the type of an application.
    ///
    /// If an application is well-typed, there must exist a type `X` such that the first term has
    /// type `T -> X` and the second term has type `T`.
    ///
    /// This method introduces a new type variable `X` and adds the constraint `T1 = T2 -> X` where
    /// `T1` is `t1`'s type and `T2` is `t2`'s type. The returned type is `X`.
    fn type_of_app(
        &mut self,
        loc: Location,
        t1: &Located<Term<'a>>,
        t2: &Located<Term<'a>>,
    ) -> TyResult<Located<Ty>> {
        let ty1 = self.type_of(t1)?.content;
        let ty2 = self.type_of(t2)?;
        let ty = self.new_ty();

        self.add_constraint(
            ty1,
            Ty::Arrow(Box::new(ty2.content), Box::new(ty.clone())),
            ty2.loc,
        );

        Ok(loc.with_content(ty))
    }

    /// Returns the type of a let binding.
    ///
    /// Typing a let binding requires adding a type binding for the name in the context. The name
    /// is binded to whatever the type of the first term is. Then, the type of the let binding is
    /// the same as the type of the second term.
    ///
    /// If the user provided a type annotation, the inferred type for the first name must coincide
    /// with such annotation and a constraint is added accordingly.
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
                    self.add_constraint(ty.content.clone(), ty1.content.clone(), ty1.loc);
                }

                self.inner.push(TyBinding {
                    name: name.content,
                    ty: ty1.content,
                });
            }
            LetKind::Rec(ty) => {
                self.inner.push(TyBinding {
                    name: name.content,
                    ty: ty.content.clone(),
                });

                // FIXME: Should a recursive function match the type of its signature? It is even
                // possible that the body has a different type and still satisfies all the
                // constraints?
                self.type_of(t1)?;
            }
        };

        let ty2 = self.type_of(t2)?.content;
        self.inner.pop().unwrap();
        Ok(Located::new(ty2, loc))
    }

    /// Returns the type of a conditional.
    ///
    /// Typing a conditional requires that the condition has type `Bool` and that both branches
    /// have the same type, both constraints are added accordingly. The returned type is the one of
    /// the first branch.
    fn type_of_cond(
        &mut self,
        loc: Location,
        t1: &Located<Term<'a>>,
        t2: &Located<Term<'a>>,
        t3: &Located<Term<'a>>,
    ) -> TyResult<Located<Ty>> {
        let ty1 = self.type_of(t1)?;
        let ty2 = self.type_of(t2)?.content;
        let ty3 = self.type_of(t3)?;

        self.add_constraint(Ty::Bool, ty1.content, ty1.loc);
        self.add_constraint(ty2.clone(), ty3.content, ty3.loc);

        Ok(loc.with_content(ty2))
    }

    /// Returns the type of a sequence.
    ///
    /// Typing a sequence adds a constraint enforcing that the first term has type `Unit`. This is
    /// because terms cannot be simply omitted during evaluation (this is a limitation of the LIR).
    /// The returned type is the same as the type of the second term.
    fn type_of_seq(
        &mut self,
        _loc: Location,
        t1: &Located<Term<'a>>,
        t2: &Located<Term<'a>>,
    ) -> TyResult<Located<Ty>> {
        let ty1 = self.type_of(t1)?;
        self.add_constraint(Ty::Unit, ty1.content, ty1.loc);
        // FIXME: this is the only method that doesn't use the location of the Term to reflect its
        // own location. If we can this, all the `type_of_*` methods could return `TyResult<Ty>`
        self.type_of(t2)
    }

    /// Returns the type of a primitive function.
    ///
    /// The typing rules for each primitive are the following:
    ///
    /// - The `print` function has type `X -> Unit` for any `X`. Thus, a new variable is added to
    /// the typing context to represent this `X`.
    fn type_of_prim_fn(&mut self, loc: Location, prim: Primitive) -> TyResult<Located<Ty>> {
        let ty = match prim {
            Primitive::Print => {
                let ty = self.new_ty();
                Ty::Arrow(Box::new(ty), Box::new(Ty::Unit))
            }
        };
        Ok(loc.with_content(ty))
    }
}
