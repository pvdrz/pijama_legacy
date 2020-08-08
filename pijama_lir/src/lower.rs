use pijama_hir::{BindKind, Term as HirTerm, LocalId, TermKind};

use crate::Term;

pub fn remove_names(term: HirTerm) -> Term {
    Context::default().remove_names(term)
}

#[derive(Default)]
struct Context {
    inner: Vec<LocalId>,
}

impl Context {
    fn remove_names(&mut self, term: HirTerm) -> Term {
        match term.kind {
            TermKind::Lit(lit) => lit.into(),
            TermKind::Var(name) => {
                let (index, _) = self
                    .inner
                    .iter()
                    .rev()
                    .enumerate()
                    .find(|(_, name2)| name == **name2)
                    .unwrap();
                Term::Var(index)
            }
            TermKind::Abs(name, body) => {
                self.inner.push(name);
                let body = self.remove_names(*body);
                self.inner.pop().unwrap();
                Term::Abs(Box::new(body))
            }
            TermKind::UnaryOp(op, t1) => {
                let t1 = self.remove_names(*t1);
                Term::UnaryOp(op, Box::new(t1))
            }
            TermKind::BinaryOp(op, t1, t2) => {
                let t1 = self.remove_names(*t1);
                let t2 = self.remove_names(*t2);
                Term::BinaryOp(op, Box::new(t1), Box::new(t2))
            }
            TermKind::App(t1, t2) => {
                let t1 = self.remove_names(*t1);
                let t2 = self.remove_names(*t2);
                Term::App(Box::new(t1), Box::new(t2))
            }
            TermKind::Let(kind, name, t1, t2) => {
                let t1 = if let BindKind::Rec = kind {
                    // if the let binding is recursive we are dealing with a recursive function and
                    // we need its name inside the context to lower its body.
                    //
                    // Also the indices must be shifted by one because the function will be wrapped
                    // in an additional abstraction.
                    //
                    // Both things are satisfied by just pushing the name of the function into the
                    // context.
                    self.inner.push(name);
                    Term::Fix(Box::new(Term::Abs(Box::new(self.remove_names(*t1)))))
                } else {
                    // if the let binding is non-recursive, we first lower the binded term, and
                    // then we make its name availabe by pushing it into the context
                    let t1 = self.remove_names(*t1);
                    self.inner.push(name);
                    t1
                };

                let t2 = self.remove_names(*t2);
                self.inner.pop().unwrap();
                Term::App(Box::new(Term::Abs(Box::new(t2))), Box::new(t1))
            }
            TermKind::Cond(t1, t2, t3) => {
                let t1 = self.remove_names(*t1);
                let t2 = self.remove_names(*t2);
                let t3 = self.remove_names(*t3);
                Term::Cond(Box::new(t1), Box::new(t2), Box::new(t3))
            }
            TermKind::PrimFn(prim) => Term::PrimFn(prim),
        }
    }
}
