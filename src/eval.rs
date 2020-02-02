use std::collections::HashMap;

use crate::ir::{Expr, FuncId, Op, Symbol};
use crate::ty::Ty;

#[derive(Default)]
pub struct Interpreter {
    funcs: HashMap<FuncId, Expr>,
}

impl Interpreter {
    fn get_func(&self, func_id: FuncId) -> Expr {
        self.funcs.get(&func_id).unwrap().clone()
    }

    pub fn eval(&mut self, expr: &Expr) -> Expr {
        println!("eval: {:?}", expr);
        match expr {
            Expr::Application { func, args } => self.eval_application(func, args),
            Expr::FnDef { id, body } => self.eval_fn_def(*id, body),
            Expr::Switch {
                target,
                branches,
                default,
            } => {
                let target = self.eval(target);
                for (pattern, result) in branches {
                    if target == self.eval(pattern) {
                        return self.eval(result);
                    }
                }
                self.eval(default)
            }
            _ => expr.clone(),
        }
    }

    fn eval_fn_def(&mut self, func_id: FuncId, body: &Expr) -> Expr {
        self.funcs.insert(func_id, body.clone());
        Expr::Symbol(Symbol::Func(func_id))
    }

    fn eval_application(&mut self, func: &Expr, args: &[Expr]) -> Expr {
        match self.eval(func) {
            Expr::Symbol(Symbol::Func(func_id)) => {
                let mut func = self.get_func(func_id);
                for (i, arg) in args.iter().enumerate() {
                    let arg = self.eval(arg);
                    func.replace(&Expr::Symbol(Symbol::Arg(i, func_id)), &arg);
                }
                self.eval(&func)
            }
            Expr::Symbol(Symbol::Operator(op)) => {
                let (a, b) = match args {
                    [a, b] => (self.eval(a), self.eval(b)),
                    _ => unreachable!(),
                };

                let a = match a {
                    Expr::Symbol(Symbol::Literal { bits, .. }) => bits,
                    _ => unreachable!(),
                };

                let b = match b {
                    Expr::Symbol(Symbol::Literal { bits, .. }) => bits,
                    _ => unreachable!(),
                };

                match op {
                    Op::Add => Expr::Symbol(Symbol::Literal {
                        bits: a + b,
                        ty: Ty::Int,
                    }),
                    Op::Sub => Expr::Symbol(Symbol::Literal {
                        bits: a - b,
                        ty: Ty::Int,
                    }),
                    Op::Mul => Expr::Symbol(Symbol::Literal {
                        bits: a * b,
                        ty: Ty::Int,
                    }),
                    Op::Div => Expr::Symbol(Symbol::Literal {
                        bits: a / b,
                        ty: Ty::Int,
                    }),
                    Op::Eq => Expr::Symbol(Symbol::Literal {
                        bits: (a == b) as u64,
                        ty: Ty::Bool,
                    }),
                }
            }
            _ => unreachable!(),
        }
    }
}
