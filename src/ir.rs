use std::convert::{TryFrom, TryInto};

use crate::parser::ASTNode;

use self::result::*;

mod result;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Ty {
    Bool,
    Int,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Symbol<'a> {
    Name(&'a str),
    Operator(Op),
    Literal { bits: u64, ty: Ty },
}

impl<'a> Symbol<'a> {
    const fn bool(input: bool) -> Self {
        Symbol::Literal {
            bits: input as u64,
            ty: Ty::Bool,
        }
    }

    const fn int(input: i64) -> Self {
        Symbol::Literal {
            bits: input as u64,
            ty: Ty::Int,
        }
    }
}

#[derive(Debug)]
pub enum Expr<'a> {
    FnDef {
        name: &'a str,
        args: Vec<&'a str>,
        body: Box<Expr<'a>>,
    },
    Switch {
        target: Box<Expr<'a>>,
        patterns: Vec<(Expr<'a>, Expr<'a>)>,
        default: Box<Expr<'a>>,
    },
    Application {
        func: Box<Expr<'a>>,
        args: Vec<Expr<'a>>,
    },
    Symbol(Symbol<'a>),
}

impl<'a> Expr<'a> {
    fn try_symbol_from(node: &ASTNode<'a>) -> LowerResult<Option<Self>> {
        let name = if let ASTNode::Atom(name) = node {
            *name
        } else {
            return Ok(None);
        };

        let symbol = match name {
            "true" => Symbol::bool(true),
            "false" => Symbol::bool(false),
            "+" => Symbol::Operator(Op::Add),
            "-" => Symbol::Operator(Op::Sub),
            "*" => Symbol::Operator(Op::Mul),
            "/" => Symbol::Operator(Op::Div),
            "=" => Symbol::Operator(Op::Eq),
            _ => {
                if let Ok(n) = name.parse::<i64>() {
                    Symbol::int(n)
                } else {
                    Symbol::Name(name)
                }
            }
        };

        Ok(Some(Expr::Symbol(symbol)))
    }

    fn try_fn_def_from(node: &ASTNode<'a>) -> LowerResult<Option<Self>> {
        let seq = if let ASTNode::Seq(seq) = node {
            if let Some(ASTNode::Atom("defun")) = seq.get(0) {
                seq
            } else {
                return Ok(None);
            }
        } else {
            return Ok(None);
        };

        let name = if let Some(name) = seq.get(1) {
            if let ASTNode::Atom(name) = name {
                *name
            } else {
                Err(LowerError::NotAtom)?
            }
        } else {
            Err(LowerError::MissingExpr)?
        };

        let args = if let Some(args) = seq.get(2) {
            if let ASTNode::Seq(args) = args {
                let mut names = Vec::new();
                for arg in args {
                    if let ASTNode::Atom(arg) = arg {
                        names.push(*arg);
                    } else {
                        Err(LowerError::NotAtom)?
                    }
                }
                names
            } else {
                Err(LowerError::NotSeq)?
            }
        } else {
            Err(LowerError::MissingExpr)?
        };

        let body = if let Some(body) = seq.get(3) {
            Box::new(Self::try_from(body)?)
        } else {
            Err(LowerError::MissingExpr)?
        };

        Ok(Some(Expr::FnDef { name, args, body }))
    }

    fn try_switch_from(node: &ASTNode<'a>) -> LowerResult<Option<Self>> {
        let seq = if let ASTNode::Seq(seq) = node {
            if let Some(ASTNode::Atom("switch")) = seq.get(0) {
                seq
            } else {
                return Ok(None);
            }
        } else {
            return Ok(None);
        };

        let target = if let Some(target) = seq.get(1) {
            Box::new(Self::try_from(target)?)
        } else {
            Err(LowerError::MissingExpr)?
        };

        let (patterns, default) = if let Some(seq) = seq.get(2..) {
            let mut patterns = Vec::new();

            let default = if let Some(default) = seq.last() {
                Box::new(Self::try_from(default)?)
            } else {
                Err(LowerError::MissingExpr)?
            };

            for node in &seq[..seq.len() - 1] {
                if let ASTNode::Seq(pattern) = node {
                    if pattern.len() != 2 {
                        Err(LowerError::MalformedExpr)?
                    }
                    patterns.push((
                        pattern.get(0).unwrap().try_into()?,
                        pattern.get(1).unwrap().try_into()?,
                    ));
                } else {
                    Err(LowerError::NotSeq)?
                }
            }
            (patterns, default)
        } else {
            Err(LowerError::MissingExpr)?
        };

        Ok(Some(Expr::Switch {
            target,
            patterns,
            default,
        }))
    }

    fn try_switch_from_if(node: &ASTNode<'a>) -> LowerResult<Option<Self>> {
        let seq = if let ASTNode::Seq(seq) = node {
            if let Some(ASTNode::Atom("if")) = seq.get(0) {
                seq
            } else {
                return Ok(None);
            }
        } else {
            return Ok(None);
        };

        let target = if let Some(target) = seq.get(1) {
            Box::new(Self::try_from(target)?)
        } else {
            Err(LowerError::MissingExpr)?
        };

        let patterns = if let Some(node) = seq.get(2) {
            vec![(Self::Symbol(Symbol::bool(true)), Self::try_from(node)?)]
        } else {
            Err(LowerError::MissingExpr)?
        };

        let default = if let Some(node) = seq.get(3) {
            Box::new(Self::try_from(node)?)
        } else {
            Err(LowerError::MissingExpr)?
        };

        Ok(Some(Expr::Switch {
            target,
            patterns,
            default,
        }))
    }

    fn try_application_from(node: &ASTNode<'a>) -> LowerResult<Option<Self>> {
        let seq = if let ASTNode::Seq(seq) = node {
            if !seq.is_empty() {
                seq
            } else {
                return Ok(None);
            }
        } else {
            return Ok(None);
        };

        let func = Box::new(Self::try_from(seq.get(0).unwrap())?);
        let args = if let Some(args) = seq.get(1..) {
            args.iter()
                .map(Self::try_from)
                .collect::<LowerResult<Vec<_>>>()?
        } else {
            Vec::new()
        };

        Ok(Some(Expr::Application { func, args }))
    }
}

impl<'a> TryFrom<&ASTNode<'a>> for Expr<'a> {
    type Error = LowerError;

    fn try_from(node: &ASTNode<'a>) -> LowerResult<Self> {
        let expr = if let Some(expr) = Self::try_symbol_from(node)? {
            expr
        } else if let Some(expr) = Self::try_fn_def_from(node)? {
            expr
        } else if let Some(expr) = Self::try_switch_from(node)? {
            expr
        } else if let Some(expr) = Self::try_switch_from_if(node)? {
            expr
        } else if let Some(expr) = Self::try_application_from(node)? {
            expr
        } else {
            return Err(LowerError::MalformedExpr);
        };
        Ok(expr)
    }
}
