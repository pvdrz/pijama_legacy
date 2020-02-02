use crate::ty::Ty;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct FuncId(pub usize);

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Symbol {
    Operator(Op),
    Literal { bits: u64, ty: Ty },
    Arg(usize, FuncId),
    Func(FuncId),
    Nil,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expr {
    FnDef {
        id: FuncId,
        body: Box<Expr>,
    },
    Switch {
        target: Box<Expr>,
        branches: Vec<(Expr, Expr)>,
        default: Box<Expr>,
    },
    Application {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    Symbol(Symbol),
}

impl Expr {
    pub fn replace(&mut self, targ: &Expr, subs: &Expr) {
        if self == targ {
            *self = subs.clone();
        } else {
            match self {
                Expr::FnDef { body, .. } => body.replace(targ, subs),
                Expr::Switch {
                    target,
                    branches,
                    default,
                } => {
                    target.replace(targ, subs);
                    for (pattern, result) in branches {
                        pattern.replace(targ, subs);
                        result.replace(targ, subs);
                    }
                    default.replace(targ, subs);
                }
                Expr::Application { func, args } => {
                    func.replace(targ, subs);
                    for arg in args {
                        arg.replace(targ, subs);
                    }
                }
                _ => (),
            }
        }
    }
}
