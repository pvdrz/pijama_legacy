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
            let Constraint { lhs: s, rhs: t } = constr.content;

            if s == t {
                return self.unify();
            }

            if let Ty::Var(_) = s {
                if !t.contains(&s) {
                    let subst = Substitution::new(s, t);
                    self.apply_substitution(&subst);
                    self.unify()?;
                    self.add_substitution(subst);
                    return Ok(());
                }
            }

            if let Ty::Var(_) = t {
                if !s.contains(&t) {
                    let subst = Substitution::new(t, s);
                    self.apply_substitution(&subst);
                    self.unify()?;
                    self.add_substitution(subst);
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
