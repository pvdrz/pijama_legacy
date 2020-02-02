use crate::ast::Node;
use crate::ir::{Expr, FuncId, Op, Symbol};
use crate::ty::Ty;

use super::scope::ScopeArena;
use super::{LowerError, LowerResult};

#[derive(Default)]
pub struct Context<'a> {
    scopes: ScopeArena<'a>,
    func_counter: usize,
}

impl<'a> Context<'a> {
    fn new_func_id(&mut self) -> FuncId {
        let id = FuncId(self.func_counter);
        self.func_counter += 1;
        id
    }

    pub fn lower(&mut self, node: &Node<'a>) -> LowerResult<'a, Expr> {
        match node {
            Node::Atom(name) => {
                let symbol = match *name {
                    "true" => Symbol::Literal {
                        bits: true as u64,
                        ty: Ty::Bool,
                    },
                    "false" => Symbol::Literal {
                        bits: true as u64,
                        ty: Ty::Bool,
                    },
                    "+" => Symbol::Operator(Op::Add),
                    "-" => Symbol::Operator(Op::Sub),
                    "*" => Symbol::Operator(Op::Mul),
                    "/" => Symbol::Operator(Op::Div),
                    "=" => Symbol::Operator(Op::Eq),
                    name => {
                        if let Ok(bits) = name.parse() {
                            Symbol::Literal { bits, ty: Ty::Int }
                        } else if let Some(symbol) = self.scopes.resolve(name) {
                            symbol.clone()
                        } else {
                            return Err(LowerError::UndefinedAtom(name));
                        }
                    }
                };
                Ok(Expr::Symbol(symbol))
            }
            Node::Seq(seq) => {
                if let Some(node) = seq.first() {
                    match node {
                        Node::Atom("defun") => {
                            if let [name, args, body] = &seq[1..] {
                                let name = name.try_name().ok_or_else(|| LowerError::Custom(format!("A function's name must be an atom")))?;
                                let args = args.try_seq().ok_or_else(|| LowerError::Custom(format!("A function's arguments must be a sequence")))?;
                                self.scopes.down();
                                let func_id = if let Some(symbol) = self.scopes.resolve(name) {
                                    if let Symbol::Func(id) = symbol {
                                        *id
                                    } else {
                                        return Err(LowerError::Custom(format!("Expected function")));
                                    }
                                } else {
                                    let id = self.new_func_id();
                                    self.scopes.define(name, Symbol::Func(id));
                                    id
                                };

                                self.scopes.down();

                                for (index, arg) in args.iter().enumerate() {
                                    let name = arg.try_name().ok_or_else(|| LowerError::Custom(format!("A function's argument must be an atom")))?;
                                    self.scopes.define(name, Symbol::Arg(index, func_id));
                                }

                                let expr = Ok(Expr::FnDef {
                                    id: func_id,
                                    body: Box::new(self.lower(body)?),
                                });

                                self.scopes.up();

                                expr
                            } else {
                                return Err(LowerError::ArityMismatch("defun", 3, seq.len() - 1));
                            }
                        },

                        Node::Atom("if") => {
                            if let [cond, then, otherwise] = &seq[1..] {
                                let target = Box::new(self.lower(cond)?);
                                let branches = vec![(
                                    self.lower(&Node::Atom("true"))?,
                                    self.lower(then)?,
                                )];
                                let default = Box::new(self.lower(otherwise)?);

                                Ok(Expr::Switch {
                                    target,
                                        branches,
                                    default,
                                })
                            } else {
                                return Err(LowerError::ArityMismatch("if", 3, seq.len() - 1));
                            }
                        },
                        func /* Must be func application */ => {
                            let func = self.lower(func)?;
                            let args = seq.iter().skip(1).map(|expr| {
                                self.lower(expr)
                            }).collect::<LowerResult<'a, _>>()?;

                            Ok(Expr::Application {
                                func: Box::new(func),
                                args,
                            })
                        }
                    }
                } else {
                    Ok(Expr::Symbol(Symbol::Nil))
                }
            }
        }
    }
}
