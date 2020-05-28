use crate::ty::Ty;

/// Represents a substitution rule over types.
pub struct Substitution {
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
pub struct Constraint {
    rhs: Ty,
    lhs: Ty,
}

impl Constraint {
    /// Creates a new constraint.
    pub fn new(rhs: Ty, lhs: Ty) -> Self {
        Constraint { rhs, lhs }
    }

    /// Consumes a constraint, returning both sides of it.
    pub fn consume(self) -> (Ty, Ty) {
        (self.rhs, self.lhs)
    }
}
