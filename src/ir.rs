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

#[derive(Debug)]
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
