use crate::parser::ASTNode;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Ty {
    Bool,
    Int,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Symbol<'a> {
    Name(&'a str),
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
    fn symbol_from(node: &ASTNode<'a>) -> Option<Self> {
        let name = if let ASTNode::Atom(name) = node {
            *name
        } else {
            return None;
        };

        let symbol = if name == "true" {
            Symbol::bool(true)
        } else if name == "false" {
            Symbol::bool(false)
        } else if let Ok(n) = name.parse::<i64>() {
            Symbol::int(n)
        } else {
            Symbol::Name(name)
        };

        Some(Expr::Symbol(symbol))
    }

    fn fn_def_from(node: &ASTNode<'a>) -> Option<Self> {
        let seq = if let ASTNode::Seq(seq) = node {
            if let Some(ASTNode::Atom("defun")) = seq.get(0) {
                seq
            } else {
                return None;
            }
        } else {
            return None;
        };

        let name = if let Some(name) = seq.get(1) {
            if let ASTNode::Atom(name) = name {
                *name
            } else {
                panic!("name is not an atom");
            }
        } else {
            panic!("no name");
        };

        let args = if let Some(args) = seq.get(2) {
            if let ASTNode::Seq(args) = args {
                let mut names = Vec::new();
                for arg in args {
                    if let ASTNode::Atom(arg) = arg {
                        names.push(*arg);
                    } else {
                        panic!("arg is not an atom");
                    }
                }
                names
            } else {
                panic!("args are not a seq");
            }
        } else {
            panic!("no args");
        };

        let body = if let Some(body) = seq.get(3) {
            Box::new(Self::from(body))
        } else {
            panic!("no body");
        };

        Some(Expr::FnDef { name, args, body })
    }

    fn switch_from(node: &ASTNode<'a>) -> Option<Self> {
        let seq = if let ASTNode::Seq(seq) = node {
            if let Some(ASTNode::Atom("switch")) = seq.get(0) {
                seq
            } else {
                return None;
            }
        } else {
            return None;
        };

        let target = if let Some(target) = seq.get(1) {
            Box::new(Self::from(target))
        } else {
            panic!("no target");
        };

        let (patterns, default) = if let Some(seq) = seq.get(2..) {
                let mut patterns = Vec::new();

                let default = if let Some(default) = seq.last() {
                    Box::new(Self::from(default))
                } else {
                    panic!("no default");
                };

                for node in &seq[..seq.len() - 1] {
                    if let ASTNode::Seq(pattern) = node {
                        if pattern.len() != 2 {
                            panic!("pattern has wrong size");
                        }
                        patterns.push((
                            pattern.get(0).unwrap().into(),
                            pattern.get(1).unwrap().into(),
                        ));
                    } else {
                        println!("{:?}", node);
                        panic!("pattern is not a seq");
                    }
                }
                (patterns, default)

        } else {
            panic!("no patterns");
        };


        Some(Expr::Switch {
            target,
            patterns,
            default,
        })
    }

    fn application_from(node: &ASTNode<'a>) -> Option<Self> {
        let seq = if let ASTNode::Seq(seq) = node {
            if !seq.is_empty() {
                seq
            } else {
                return None;
            }
        } else {
            return None;
        };

        let func = Box::new(Self::from(seq.get(0).unwrap()));
        let args = if let Some(args) = seq.get(1..) {
            args.iter().map(Expr::from).collect()
        } else {
            Vec::new()
        };

        Some(Expr::Application { func, args })
    }
}

impl<'a> From<&ASTNode<'a>> for Expr<'a> {
    fn from(node: &ASTNode<'a>) -> Self {
        Self::symbol_from(node)
            .or_else(|| Self::fn_def_from(node))
            .or_else(|| Self::switch_from(node))
            .or_else(|| Self::application_from(node))
            .expect("no matching rule")
    }
}
