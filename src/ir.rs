use crate::parser::ASTNode;

#[derive(Debug)]
pub enum Ty {
    Bool,
    Int,
    Unit,
}

#[derive(Debug)]
pub enum Symbol<'a> {
    Name(&'a str),
    Literal {
        bits: u64,
        ty: Ty,
    }
}

impl<'a> Symbol<'a> {
    const fn bool(input: bool) -> Self {
        Symbol::Literal {
            bits: input as u64,
            ty: Ty::Bool
        }
    }

    const fn int(input: i64) -> Self {
        Symbol::Literal {
            bits: input as u64,
            ty: Ty::Int
        }
    }

    const fn nil() -> Self {
        Symbol::Literal {
            bits: 0,
            ty: Ty::Unit,
        }
    }
}

#[derive(Debug)]
pub enum Expr<'a> {
    Atom(Symbol<'a>),
    Cons(Box<Expr<'a>>, Box<Expr<'a>>),
}

impl<'a> From<ASTNode<'a>> for Expr<'a> {
    fn from(node: ASTNode<'a>) -> Self {
        match node {
            ASTNode::Atom(s) => {
                let symbol = if s == "true" {
                    Symbol::bool(true)
                } else if s == "false" {
                    Symbol::bool(false)
                } else if let Ok(n) = s.parse::<i64>() {
                    Symbol::int(n)
                } else {
                    Symbol::Name(s)
                };
                Expr::Atom(symbol)
            }

            ASTNode::Seq(mut nodes) => {
                let mut expr = Expr::Atom(Symbol::nil());
                for node in nodes.drain(..).rev() {
                    expr = Expr::Cons(Box::new(node.into()), Box::new(expr));
                };
                expr
            }
        }
    }
}
