use pijama_ast::Located;
use crate::ty::{ty_check::Context, Ty, TyError, TyResult};

pub struct Unifier {
    substitutions: Vec<Substitution>,
    constraints: Vec<Located<Constraint>>,
}

impl Unifier {
    pub(super) fn from_ctx<'a>(ctx: Context<'a>) -> TyResult<Self> {
        let mut unif = Unifier {
            constraints: ctx.constraints,
            substitutions: Default::default(),
        };
        unif.unify()?;
        Ok(unif)
    }

    pub(super) fn replace(&self, ty: &mut Ty) {
        for subst in &self.substitutions {
            subst.apply(ty);
        }
    }

    fn apply_substitution(&mut self, subst: &Substitution) {
        for constr in &mut self.constraints {
            let Constraint { lhs, rhs } = &mut constr.content;
            subst.apply(lhs);
            subst.apply(rhs);
        }
    }

    fn add_substitution(&mut self, mut subst: Substitution) {
        self.replace(&mut subst.new);
        self.substitutions.push(subst);
    }

    fn unify(&mut self) -> TyResult<()> {
        if let Some(constr) = self.constraints.pop() {
            let loc = constr.loc;
            let Constraint { lhs, rhs } = constr.content;

            match (lhs, rhs) {
                (lhs, rhs) if lhs == rhs => self.unify()?,

                (lhs @ Ty::Var(_), rhs) if !rhs.contains(&lhs) => {
                    let subst = Substitution::new(lhs, rhs);
                    self.apply_substitution(&subst);
                    self.unify()?;
                    self.add_substitution(subst);
                }

                (lhs, rhs @ Ty::Var(_)) if !lhs.contains(&rhs) => {
                    let subst = Substitution::new(rhs, lhs);
                    self.apply_substitution(&subst);
                    self.unify()?;
                    self.add_substitution(subst);
                }

                (Ty::Arrow(s1, s2), Ty::Arrow(t1, t2)) => {
                    self.constraints
                        .push(Located::new(Constraint::new(*s1, *t1), loc));
                    self.constraints
                        .push(Located::new(Constraint::new(*s2, *t2), loc));
                    self.unify()?;
                }

                (lhs, rhs) => {
                    return Err(TyError::Unexpected {
                        expected: lhs,
                        found: Located::new(rhs, loc),
                    });
                }
            }
        }
        Ok(())
    }
}

/// Represents a substitution rule over types.
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
    lhs: Ty,
    rhs: Ty,
}

impl Constraint {
    /// Creates a new constraint.
    pub fn new(lhs: Ty, rhs: Ty) -> Self {
        Constraint { lhs, rhs }
    }
}
