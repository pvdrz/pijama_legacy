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

type Block<'a> = Vec<Node<'a>>;

pub fn lower<'a>(blk: Block<'a>) -> Term<'a> {
    lower_blk(blk)
}

fn lower_blk<'a>(blk: Block<'a>) -> Term<'a> {
    let mut terms = blk.into_iter().rev().map(lower_node);
    if let Some(mut term) = terms.next() {
        for prev_term in terms {
            let next_term = Box::new(term);
            term = if let Term::Let(name, value, _) = prev_term {
                Term::Let(name, value, next_term)
            } else {
                Term::Seq(Box::new(prev_term), next_term)
            };
        }
        term
    } else {
        Term::Lit(Literal::Unit)
    }
}

fn lower_node<'a>(node: Node<'a>) -> Term<'a> {
    match node {
        Node::Name(name) => Term::Var(name),
        Node::Cond(if_blk, do_blk, el_blk) => lower_cond(if_blk, do_blk, el_blk),
        Node::Literal(lit) => Term::Lit(lit),
        Node::Call(name, args) => lower_call(name, args),
        Node::BinaryOp(bin_op, node1, node2) => lower_binary_op(bin_op, *node1, *node2),
        Node::UnaryOp(un_op, node) => lower_unary_op(un_op, *node),
        Node::LetBind(name, node) => lower_let_bind(name, *node),
        Node::FnDef(name, binds, body) => lower_fn_def(name, binds, body),
    }
}

fn lower_cond<'a>(if_blk: Block<'a>, do_blk: Block<'a>, el_blk: Block<'a>) -> Term<'a> {
    Term::Cond(
        Box::new(lower_blk(if_blk)),
        Box::new(lower_blk(do_blk)),
        Box::new(lower_blk(el_blk)),
    )
}

fn lower_call<'a>(name: Name<'a>, args: Block<'a>) -> Term<'a> {
    let mut term = Term::Var(name);
    for node in args {
        term = Term::App(Box::new(term), Box::new(lower_node(node)));
    }
    term
}

fn lower_binary_op<'a>(bin_op: BinOp, node1: Node<'a>, node2: Node<'a>) -> Term<'a> {
    Term::App(
        Box::new(Term::App(
            Box::new(Term::Abs(Abstraction::Binary(bin_op))),
            Box::new(lower_node(node1)),
        )),
        Box::new(lower_node(node2)),
    )
}

fn lower_unary_op<'a>(un_op: UnOp, node: Node<'a>) -> Term<'a> {
    Term::App(
        Box::new(Term::Abs(Abstraction::Unary(un_op))),
        Box::new(lower_node(node)),
    )
}

fn lower_let_bind<'a>(name: Name<'a>, node: Node<'a>) -> Term<'a> {
    Term::Let(
        name,
        Box::new(lower_node(node)),
        Box::new(Term::Lit(Literal::Unit)),
    )
}

fn lower_fn_def<'a>(name: Name<'a>, binds: Vec<Binding<'a>>, body: Block<'a>) -> Term<'a> {
    let mut term = lower_blk(body);
    for bind in binds.into_iter().rev() {
        term = Term::Abs(Abstraction::Lambda(bind, Box::new(term)));
    }

    Term::Let(name, Box::new(term), Box::new(Term::Lit(Literal::Unit)))
}
