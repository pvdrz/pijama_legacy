use crate::ast::Name;
use crate::{lir, mir};

pub fn remove_names(term: mir::Term<'_>) -> lir::Term {
    Context::default().remove_names(term)
}

#[derive(Default)]
struct Context<'a> {
    inner: Vec<Name<'a>>,
}

impl<'a> Context<'a> {
    fn remove_names(&mut self, term: mir::Term<'a>) -> lir::Term {
        match term {
            mir::Term::Lit(literal) => lir::Term::Lit(literal),
            mir::Term::Var(name) => {
                let (index, _) = self
                    .inner
                    .iter()
                    .rev()
                    .enumerate()
                    .find(|(_, name2)| name == **name2)
                    .unwrap();
                lir::Term::Var(index)
            }
            mir::Term::Abs(bind, body) => {
                self.inner.push(bind.name);
                let body = self.remove_names(*body);
                self.inner.pop().unwrap();
                lir::Term::Abs(Box::new(body))
            }
            mir::Term::UnaryOp(op, t1) => {
                let t1 = self.remove_names(*t1);
                lir::Term::UnaryOp(op, Box::new(t1))
            }
            mir::Term::BinaryOp(op, t1, t2) => {
                let t1 = self.remove_names(*t1);
                let t2 = self.remove_names(*t2);
                lir::Term::BinaryOp(op, Box::new(t1), Box::new(t2))
            }
            mir::Term::BuiltInFn(builtin, t1) => {
                let t1 = self.remove_names(*t1);
                lir::Term::App(Box::new(lir::Term::BuiltInFn(builtin)), Box::new(t1))
            }
            mir::Term::App(t1, t2) => {
                let t1 = self.remove_names(*t1);
                let t2 = self.remove_names(*t2);
                lir::Term::App(Box::new(t1), Box::new(t2))
            }
            mir::Term::Let(name, t1, t2) => {
                let t1 = self.remove_names(*t1);
                self.inner.push(name);
                let t2 = self.remove_names(*t2);
                self.inner.pop().unwrap();
                lir::Term::App(Box::new(lir::Term::Abs(Box::new(t2))), Box::new(t1))
            }
            mir::Term::Cond(t1, t2, t3) => {
                let t1 = self.remove_names(*t1);
                let t2 = self.remove_names(*t2);
                let t3 = self.remove_names(*t3);
                lir::Term::Cond(Box::new(t1), Box::new(t2), Box::new(t3))
            }
            mir::Term::Seq(t1, t2) => {
                let t1 = self.remove_names(*t1);
                let t2 = self.remove_names(*t2);
                lir::Term::App(Box::new(lir::Term::Abs(Box::new(t2))), Box::new(t1))
            }
            mir::Term::Fix(t1) => {
                let t1 = self.remove_names(*t1);
                lir::Term::Fix(Box::new(t1))
            }
        }
    }
}
