use crate::{
    ast::{Located, Name},
    lir, mir,
};

pub fn remove_names(term: Located<mir::Term<'_>>) -> lir::Term {
    Context::default().remove_names(term.content)
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
                let body = self.remove_names(body.content);
                self.inner.pop().unwrap();
                lir::Term::Abs(Box::new(body))
            }
            mir::Term::UnaryOp(op, t1) => {
                let t1 = self.remove_names(t1.content);
                lir::Term::UnaryOp(op, Box::new(t1))
            }
            mir::Term::BinaryOp(op, t1, t2) => {
                let t1 = self.remove_names(t1.content);
                let t2 = self.remove_names(t2.content);
                lir::Term::BinaryOp(op, Box::new(t1), Box::new(t2))
            }
            mir::Term::App(t1, t2) => {
                let t1 = self.remove_names(t1.content);
                let t2 = self.remove_names(t2.content);
                lir::Term::App(Box::new(t1), Box::new(t2))
            }
            mir::Term::Let(name, t1, t2) => {
                let t1 = self.remove_names(t1.content);
                self.inner.push(name.content);
                let t2 = self.remove_names(t2.content);
                self.inner.pop().unwrap();
                lir::Term::App(Box::new(lir::Term::Abs(Box::new(t2))), Box::new(t1))
            }
            mir::Term::Cond(t1, t2, t3) => {
                let t1 = self.remove_names(t1.content);
                let t2 = self.remove_names(t2.content);
                let t3 = self.remove_names(t3.content);
                lir::Term::Cond(Box::new(t1), Box::new(t2), Box::new(t3))
            }
            mir::Term::Seq(t1, t2) => {
                let t1 = self.remove_names(t1.content);
                let t2 = self.remove_names(t2.content);
                lir::Term::App(Box::new(lir::Term::Abs(Box::new(t2))), Box::new(t1))
            }
            mir::Term::Fix(t1) => {
                let t1 = self.remove_names(t1.content);
                lir::Term::Fix(Box::new(t1))
            }
            mir::Term::PrimFn(prim) => lir::Term::PrimFn(prim),
        }
    }
}
