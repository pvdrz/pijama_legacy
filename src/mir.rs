use std::fmt;

use crate::ast::*;
use crate::ty::{Binding, Ty};

#[derive(Debug)]
pub enum Abstraction<'a> {
    Lambda(Binding<'a>, Box<Term<'a>>),
    Binary(BinOp),
    Unary(UnOp),
}

impl<'a> fmt::Display for Abstraction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Abstraction::*;
        match self {
            Lambda(Binding { name, ty }, term) => write!(f, "(λ{}:{}. {})", name.0, ty, term),
            Binary(bin_op) => write!(f, "{}", bin_op),
            Unary(un_op) => write!(f, "{}", un_op),
        }
    }
}

#[derive(Debug)]
pub enum Term<'a> {
    Var(Name<'a>),
    Abs(Abstraction<'a>),
    App(Box<Term<'a>>, Box<Term<'a>>),
    Lit(Literal),
    Cond(Box<Term<'a>>, Box<Term<'a>>, Box<Term<'a>>),
    Let(Name<'a>, Box<Term<'a>>, Box<Term<'a>>),
    Seq(Box<Term<'a>>, Box<Term<'a>>),
}

impl<'a> fmt::Display for Term<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(var) => write!(f, "{}", var.0),
            Term::Abs(abs) => write!(f, "{}", abs),
            Term::App(t1, t2) => write!(f, "({} {})", t1, t2),
            Term::Lit(literal) => write!(f, "{}", literal),
            Term::Cond(t1, t2, t3) => write!(f, "(if {} then {} else {})", t1, t2, t3),
            Term::Let(name, t1, t2) => write!(f, "(let {} = {} in {})", name.0, t1, t2),
            Term::Seq(t1, t2) => write!(f, "{} ; {}", t1, t2),
        }
    }
}

fn compile_node<'a>(node: Node<'a>) -> Term<'a> {
    match node {
        Node::Name(name) => Term::Var(name),
        Node::Cond(if_block, do_block, else_block) => Term::Cond(
            Box::new(compile_block(if_block)),
            Box::new(compile_block(do_block)),
            Box::new(compile_block(else_block)),
        ),
        Node::Literal(literal) => Term::Lit(literal),
        Node::Call(name, args) => {
            let mut term = Term::Var(name);
            for node in args {
                term = Term::App(Box::new(term), Box::new(compile_node(node)));
            }
            term
        }
        Node::BinaryOp(bin_op, node1, node2) => Term::App(
            Box::new(Term::App(
                Box::new(Term::Abs(Abstraction::Binary(bin_op))),
                Box::new(compile_node(*node1)),
            )),
            Box::new(compile_node(*node2)),
        ),
        Node::UnaryOp(un_op, node) => Term::App(
            Box::new(Term::Abs(Abstraction::Unary(un_op))),
            Box::new(compile_node(*node)),
        ),
        _ => panic!("{:?}", node),
    }
}

fn compile_block_step<'a>(node: Node<'a>, block: &mut impl Iterator<Item = Node<'a>>) -> Term<'a> {
    match node {
        Node::LetBind(name, node) => compile_let_bind(name, *node, block),
        Node::FnDef(name, binds, body) => compile_fn_def(name, binds, body, block),
        _ => compile_node(node),
    }
}

pub fn compile_block<'a>(block: Vec<Node<'a>>) -> Term<'a> {
    let mut block = block.into_iter();
    if let Some(node) = block.next() {
        let mut term = compile_block_step(node, &mut block);
        while let Some(node) = block.next() {
            term = Term::Seq(
                Box::new(term),
                Box::new(compile_block_step(node, &mut block)),
            );
        }
        term
    } else {
        Term::Lit(Literal::Unit)
    }
}

fn compile_let_bind<'a>(
    name: Name<'a>,
    node: Node<'a>,
    block: &mut impl Iterator<Item = Node<'a>>,
) -> Term<'a> {
    Term::Let(
        name,
        Box::new(compile_node(node)),
        Box::new(compile_block_step(
            block.next().unwrap_or_else(|| Node::Literal(Literal::Unit)),
            block,
        )),
    )
}

fn compile_fn_def<'a>(
    name: Name<'a>,
    binds: Vec<Binding<'a>>,
    body: Vec<Node<'a>>,
    block: &mut impl Iterator<Item = Node<'a>>,
) -> Term<'a> {
    let mut term = compile_block(body);
    for bind in binds.into_iter().rev() {
        term = Term::Abs(Abstraction::Lambda(bind, Box::new(term)));
    }

    Term::Let(
        name,
        Box::new(term),
        Box::new(compile_block_step(
            block.next().unwrap_or_else(|| Node::Literal(Literal::Unit)),
            block,
        )),
    )
}
