//! Types and functions related to type unification.
//!
//! This module takes care of resolving the constraints created by the `Context` type and producing
//! a set of substitutions that can make our program well-typed. It is perfectly possible that a
//! well-typed program still has type variables in its types.
//!
//! This algorithm is based on Chapter 22 of the _Types and Programming Languages_ book by Benjamin
//! Pierce.
use std::collections::VecDeque;

use pijama_common::location::Located;
use pijama_ty::Ty;

use crate::{Context, TyError, TyResult};

/// Solves the constraints created by the `Context` type.
///
/// This type is able to find a set of `Substitution`s such that the program that produced the
/// `Context`'s `Constraint`s is well-typed.
#[derive(Debug)]
pub struct Unifier {
    /// Substitutions that make the program well-typed.
    substitutions: Vec<Substitution>,
    /// Typing constraints of the program.
    constraints: VecDeque<Located<Constraint>>,
}

impl Unifier {
    /// Creates a new `Unifier` from a `Context`.
    ///
    /// Consumes the constraints collected by the `Context` and then tries to unify those
    /// constraints using the `unify` method. If this process is successful, a new `Unifier` is
    /// returned ready to be used to replace type variables.
    pub(super) fn from_ctx(ctx: Context) -> TyResult<Self> {
        let mut unif = Unifier {
            substitutions: Default::default(),
            constraints: ctx.constraints,
        };
        unif.unify()?;
        Ok(unif)
    }

    /// Replaces the type variables inside a type.
    ///
    /// This uses the `substitutions` field to replace type variables.
    pub(super) fn replace(&self, ty: &mut Ty) {
        for subst in &self.substitutions {
            subst.apply(ty);
        }
    }

    /// Applies a substitution over the set of constraints.
    ///
    /// This method applies `subst` over both sides of the `Constraint`s in the `constraints`
    /// field.
    fn apply_substitution(&mut self, subst: &Substitution) {
        for constr in &mut self.constraints {
            let Constraint { lhs, rhs } = &mut constr.content;
            subst.apply(lhs);
            subst.apply(rhs);
        }
    }

    /// Adds a new substitution to the solution.
    ///
    /// Composes the existing substitutions with `subst`. This is done by first applying all the
    /// existing substitutions to the `new` field of `subst` and then pushing `subst` to the
    /// `substitions` field.
    fn add_substitution(&mut self, mut subst: Substitution) {
        self.replace(&mut subst.new);
        self.substitutions.push(subst);
    }

    /// Solves the unification problem.
    ///
    /// This method is the core of this module. It takes care of populating the `substitutions`
    /// field. If this method returns without errors, the `Unifier` is ready to be used to
    /// `replace` type variables and the program can be assumed to be well-typed.
    fn unify(&mut self) -> TyResult<()> {
        // If there are constraints to be solved, take one.
        if let Some(constr) = self.constraints.pop_back() {
            let loc = constr.loc;
            let Constraint { lhs, rhs } = constr.content;

            match (lhs, rhs) {
                // If both sides of the constraint are equal, nothing needs to be done. We can skip
                // this constraint and go ahead with the other rules.
                (lhs, rhs) if lhs == rhs => self.unify()?,

                // If the left-hand side is a type variable and this variable is not on the
                // right-hand side we replace the left-hand side type by the right-hand side in all
                // the remaining constraints and add this substitution to our solution.
                (Ty::Var(index), rhs) if !rhs.contains(index) => {
                    let subst = Substitution::new(Ty::Var(index), rhs);
                    self.apply_substitution(&subst);
                    self.unify()?;
                    self.add_substitution(subst);
                }

                // If the right-hand side is a type variable and this variable is not on the
                // left-hand side we replace the right-hand side type by the left side-hand in all
                // the remaining constraints and add this substitution to our solution.
                (lhs, Ty::Var(index)) if !lhs.contains(index) => {
                    let subst = Substitution::new(Ty::Var(index), lhs);
                    self.apply_substitution(&subst);
                    self.unify()?;
                    self.add_substitution(subst);
                }

                // If both sides are arrow types, we add new constraints matching each side of the
                // arrows with their counterpart. This constraints are pushed at the back to
                // prioritize them.
                (Ty::Arrow(s1, s2), Ty::Arrow(t1, t2)) => {
                    self.constraints
                        .push_back(Located::new(Constraint::new(*s1, *t1), loc));
                    self.constraints
                        .push_back(Located::new(Constraint::new(*s2, *t2), loc));
                    self.unify()?;
                }

                // Otherwise, this constraint cannot be satisfied and we raise an error.
                (lhs, rhs) => {
                    return Err(TyError::Mismatch {
                        expected: lhs,
                        found: Located::new(rhs, loc),
                    });
                }
            }
        }
        // If there are no more constrains, we are done.
        Ok(())
    }
}

/// Represents a substitution rule over types.
#[derive(Debug)]
struct Substitution {
    /// Type to be replaced.
    old: Ty,
    /// The replacement type.
    new: Ty,
}

impl Substitution {
    /// Creates a new substitution rule.
    pub fn new(old: Ty, new: Ty) -> Self {
        Substitution { old, new }
    }

    /// Applies the substitution rule over a type, replacing all occurrences of `old` by `new`.
    pub fn apply(&self, ty: &mut Ty) {
        if *ty == self.old {
            *ty = self.new.clone();
        } else if let Ty::Arrow(ty1, ty2) = ty {
            self.apply(ty1);
            self.apply(ty2);
        }
    }
}

/// Represents a constraint between types.
#[derive(Debug)]
pub struct Constraint {
    /// Left-hand side of the constraint.
    ///
    /// It usually represents the expected type of a constraint.
    lhs: Ty,
    /// Right-hand side of the constraint.
    ///
    /// It usually represents the type found when creating a constraint.
    rhs: Ty,
}

impl Constraint {
    /// Creates a new constraint.
    pub fn new(lhs: Ty, rhs: Ty) -> Self {
        Constraint { lhs, rhs }
    }
}
