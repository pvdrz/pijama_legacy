//! Pijama's type-checker.
//!
//! This module contains all the functions and types required to do type-checking over the HIR of a
//! program. Pijama uses constraint-based typing, which is better suited for type
//! reconstruction/inference than a regular in-place enforcement of the typing rules.
//!
//! The entry-point for this module is the `ty_check` method which does the type checking of a
//! whole program. However, most of the heavy lifting is done by the `Analyzer` and `Unifier` types.
use std::collections::VecDeque;

use pijama_common::{
    location::{Located, Location},
    BinOp, Literal, Primitive, UnOp,
};

use pijama_ctx::{Context, ContextExt, LocalId, TypeInfo};
use pijama_hir::{BindKind, Term, TermKind};
use pijama_ty::Ty;

mod result;
mod unify;

pub use result::{TyError, TyErrorKind, TyResult};
use unify::{Constraint, Unifier};

/// Function that type-checks a term and returns its type.
///
/// This function must always be called in the "root" term of the program. Otherwise, the type
/// checker might not have all the bindings required to do its job.
pub fn ty_check(term: &Term, ctx: &mut Context) -> TyResult<Located<Ty>> {
    // Create a new, empty analyzer.
    let mut analyzer = Analyzer::new(ctx);
    // Obtain typing constraints and the type of `term`.
    let mut ty = analyzer.type_of(&term)?;
    // Solve the constraints using unification.
    let unif = Unifier::new(analyzer.constraints)?;
    // Apply the substitutions found during unification over the type of `term`.
    unif.replace(&mut ty.content);

    let mut id = None;
    for (local_id, ty) in ctx.iter_mut_local_types() {
        unif.replace(ty);
        if !ty.is_concrete() {
            id = Some(local_id);
            break;
        }
    }
    if let Some(id) = id {
        let loc = ctx.get_location(id).unwrap();
        return Err(TyError::new(TyErrorKind::NotConcrete, loc));
    }

    let mut id = None;
    for (term_id, ty) in ctx.iter_mut_term_types() {
        unif.replace(ty);
        if !ty.is_concrete() {
            id = Some(term_id);
            break;
        }
    }
    if let Some(id) = id {
        let loc = ctx.get_location(id).unwrap();
        return Err(TyError::new(TyErrorKind::NotConcrete, loc));
    }

    Ok(ty)
}

/// A typing analyzer.
///
/// This structure traverses the HIR of a term and generates a set of constraints that must be
/// satisfied by the term to be well-typed.
struct Analyzer<'ctx> {
    ctx: &'ctx mut Context,
    /// Typing constraints.
    ///
    /// Each typing constraint is introduced by a particular `type_of_*` method with a suitable
    /// location in case an error needs to be returned.
    constraints: VecDeque<Located<Constraint>>,
}

impl<'ctx> Analyzer<'ctx> {
    fn new(ctx: &'ctx mut Context) -> Self {
        Self {
            ctx,
            constraints: VecDeque::default(),
        }
    }
    /// Returns a new type variable.
    ///
    /// This variable is guaranteed to be different from all the other types introduced before.
    fn new_ty(&mut self) -> Ty {
        self.ctx.new_ty()
    }

    /// Adds a new `Constraint`.
    ///
    /// A new constraint must be added when it is required to enforce an specific typing rule.
    /// Calling this method will not enforce the rule instantly. It only stores the restriction
    /// inside the `Analyzer` to be solved by the `Unifier` in a posterior stage. This constraint
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
    fn type_of(&mut self, term: &Term) -> TyResult<Located<Ty>> {
        let loc = self.ctx.get_location(term.id).unwrap();
        let ty = match &term.kind {
            TermKind::Lit(lit) => self.type_of_lit(lit),
            TermKind::Var(name) => self.type_of_var(*name),
            TermKind::Abs(name, body) => self.type_of_abs(*name, body.as_ref()),
            TermKind::UnaryOp(op, term) => self.type_of_unary_op(*op, term.as_ref()),
            TermKind::BinaryOp(op, t1, t2) => self.type_of_binary_op(*op, t1.as_ref(), t2.as_ref()),
            TermKind::App(t1, t2) => self.type_of_app(t1.as_ref(), t2.as_ref()),
            TermKind::Let(kind, name, t1, t2) => {
                self.type_of_let(*kind, *name, t1.as_ref(), t2.as_ref())
            }
            TermKind::Cond(t1, t2, t3) => self.type_of_cond(t1.as_ref(), t2.as_ref(), t3.as_ref()),
            TermKind::PrimFn(prim) => self.type_of_prim_fn(*prim),
        }?;

        if let Some(info) = self.ctx.get_type_info(term.id) {
            let info_ty = info.ty.clone();
            self.add_constraint(info_ty, ty.clone(), loc);
        } else {
            self.ctx.insert_type_info(
                term.id,
                TypeInfo {
                    ty: ty.clone(),
                    loc,
                },
            )
        }

        Ok(loc.with_content(ty))
    }

    /// Returns the type of a literal.
    ///
    /// This rule does not add new constraints because the type of a literal is only decided by its
    /// variant.
    fn type_of_lit(&mut self, lit: &Literal) -> TyResult {
        let ty = match lit {
            Literal::Unit => Ty::Unit,
            Literal::Bool(_) => Ty::Bool,
            Literal::Number(_) => Ty::Int,
        };
        Ok(ty)
    }

    /// Returns the type of a variable.
    ///
    /// To type a variable, it must have been binded beforehand using a let binding or an
    /// abstraction and added to the context. If the variable is not in the current context, this
    /// method returns an error stating that the variable is unbounded.
    ///
    /// This rule does not add new constraints because the type of a variable is decided by the
    /// bindings done in the current scope.
    fn type_of_var(&mut self, local: LocalId) -> TyResult {
        if let Some(info) = self.ctx.get_type_info(local) {
            Ok(info.ty.clone())
        } else {
            panic!("Missing type info for {:?}", local)
        }
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
    fn type_of_abs(&mut self, arg: LocalId, body: &Term) -> TyResult {
        if let Some(info) = self.ctx.get_type_info(arg) {
            let arg_ty = info.ty.clone();
            let body_ty = self.type_of(body)?.content;
            Ok(Ty::Arrow(Box::new(arg_ty), Box::new(body_ty)))
        } else {
            panic!("Missing type info for {:?}", arg)
        }
    }

    /// Returns the type of an unary operation.
    ///
    /// The type of an unary operation depends on its operator:
    /// - If it is a negation, the operand must have type `Int`.
    /// - If it is a logical not, the operand must have type `Bool`.
    ///
    /// This rule adds a constraint stating that the type of the operand must match one of the
    /// types stated above. The returned type is the same type as the operand.
    fn type_of_unary_op(&mut self, op: UnOp, term: &Term) -> TyResult {
        let ty = self.type_of(term)?;
        let expected = match op {
            UnOp::Neg => Ty::Int,
            UnOp::Not => Ty::Bool,
        };
        self.add_constraint(expected, ty.content.clone(), ty.loc);
        Ok(ty.content)
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
    fn type_of_binary_op(&mut self, op: BinOp, t1: &Term, t2: &Term) -> TyResult {
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
        Ok(ty)
    }
    /// Returns the type of an application.
    ///
    /// If an application is well-typed, there must exist a type `X` such that the first term has
    /// type `T -> X` and the second term has type `T`.
    ///
    /// This method introduces a new type variable `X` and adds the constraint `T1 = T2 -> X` where
    /// `T1` is `t1`'s type and `T2` is `t2`'s type. The returned type is `X`.
    fn type_of_app(&mut self, t1: &Term, t2: &Term) -> TyResult {
        let ty1 = self.type_of(t1)?.content;
        let ty2 = self.type_of(t2)?;
        let ty = self.new_ty();

        self.add_constraint(
            ty1,
            Ty::Arrow(Box::new(ty2.content), Box::new(ty.clone())),
            ty2.loc,
        );

        Ok(ty)
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
    fn type_of_let(&mut self, kind: BindKind, lhs: LocalId, rhs: &Term, tail: &Term) -> TyResult {
        match kind {
            BindKind::NonRec => {
                let rhs_ty = self.type_of(rhs)?;

                let lhs_ty = self.ctx.get_type_info(lhs).unwrap().ty.clone();

                self.add_constraint(lhs_ty, rhs_ty.content.clone(), rhs_ty.loc);
            }
            BindKind::Rec => {
                let lhs_ty = self.ctx.get_type_info(lhs).unwrap().ty.clone();

                let rhs_ty = self.type_of(rhs)?;

                self.add_constraint(lhs_ty, rhs_ty.content.clone(), rhs_ty.loc);
            }
        };

        let tail_ty = self.type_of(tail)?.content;
        Ok(tail_ty)
    }

    /// Returns the type of a conditional.
    ///
    /// Typing a conditional requires that the condition has type `Bool` and that both branches
    /// have the same type, both constraints are added accordingly. The returned type is the one of
    /// the first branch.
    fn type_of_cond(&mut self, t1: &Term, t2: &Term, t3: &Term) -> TyResult {
        let ty1 = self.type_of(t1)?;
        let ty2 = self.type_of(t2)?.content;
        let ty3 = self.type_of(t3)?;

        self.add_constraint(Ty::Bool, ty1.content, ty1.loc);
        self.add_constraint(ty2.clone(), ty3.content, ty3.loc);

        Ok(ty2)
    }

    /// Returns the type of a primitive function.
    ///
    /// The typing rules for each primitive are the following:
    ///
    /// - The `print` function has type `X -> Unit` for any `X`. Thus, a new variable is added to
    /// the typing context to represent this `X`.
    fn type_of_prim_fn(&mut self, prim: Primitive) -> TyResult {
        let ty = match prim {
            Primitive::Print => {
                let ty = self.new_ty();
                Ty::Arrow(Box::new(ty), Box::new(Ty::Unit))
            }
        };
        Ok(ty)
    }
}
