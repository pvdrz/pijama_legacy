use pijama_ast::Located;
use crate::ty::{ty_check::Context, Ty, TyError, TyResult};

pub struct Unifier {
    substitution: Vec<(Ty, Ty)>,
    constraints: Vec<Located<Constraint>>,
}

impl Unifier {
    pub(super) fn from_ctx<'a>(ctx: Context<'a>) -> TyResult<Self> {
        let mut unif = Unifier {
            constraints: ctx.constraints,
            substitution: Default::default(),
        };
        unif.unify()?;
        Ok(unif)
    }

    pub(super) fn replace(&self, ty: &mut Ty) {
        for (target, subs) in &self.substitution {
            ty.replace(target, subs);
        }
    }

    fn apply_subs(&mut self, target: &Ty, subs: &Ty) {
        for constr in &mut self.constraints {
            let Constraint { lhs, rhs } = &mut constr.content;
            lhs.replace(target, subs);
            rhs.replace(target, subs);
        }
    }

    fn compose_subs(&mut self, target: Ty, mut subs: Ty) {
        self.replace(&mut subs);
        self.substitution.push((target, subs));
    }

    fn unify(&mut self) -> TyResult<()> {
        if let Some(constr) = self.constraints.pop() {
            let loc = constr.loc;
            let Constraint { lhs: s, rhs: t } = constr.content;

            if s == t {
                return self.unify();
            }

            if let Ty::Var(_) = s {
                if !t.contains(&s) {
                    self.apply_subs(&s, &t);
                    self.unify()?;
                    self.compose_subs(s, t);
                    return Ok(());
                }
            }

            if let Ty::Var(_) = t {
                if !s.contains(&t) {
                    self.apply_subs(&t, &s);
                    self.unify()?;
                    self.compose_subs(t, s);
                    return Ok(());
                }
            }

            if let (Ty::Arrow(s1, s2), Ty::Arrow(t1, t2)) = (s.clone(), t.clone()) {
                self.constraints
                    .push(Located::new(Constraint::new(*s1, *t1), loc));
                self.constraints
                    .push(Located::new(Constraint::new(*s2, *t2), loc));
                return self.unify();
            }

            return Err(TyError::Unexpected {
                expected: s,
                found: Located::new(t, loc),
            });
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

    /// Applies the substitution rule. Replacing all occurrences of `old` by `new`.
    pub fn apply(&self, ty: &mut Ty) {}
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
