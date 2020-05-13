use std::fmt;

use crate::ast::*;
use crate::ty::{Binding, Ty};

type Block<'a> = Vec<Node<'a>>;

#[derive(Debug, Clone)]
pub enum Term<'a> {
    Var(Name<'a>),
    Abs(Binding<'a>, Box<Term<'a>>),
    UnaryOp(UnOp, Box<Term<'a>>),
    BinaryOp(BinOp, Box<Term<'a>>, Box<Term<'a>>),
    App(Box<Term<'a>>, Box<Term<'a>>),
    Lit(Literal),
    Cond(Box<Term<'a>>, Box<Term<'a>>, Box<Term<'a>>),
    Let(Name<'a>, Box<Term<'a>>, Box<Term<'a>>),
    Seq(Box<Term<'a>>, Box<Term<'a>>),
    Fix(Box<Term<'a>>),
}

impl<'a> fmt::Display for Term<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(var) => write!(f, "{}", var.0),
            Term::Abs(Binding { name, ty }, term) => write!(f, "(λ{}:{}. {})", name.0, ty, term),
            Term::UnaryOp(op, term) => write!(f, "({}{})", op, term),
            Term::BinaryOp(op, t1, t2) => write!(f, "({} {} {})", t1, op, t2),
            Term::App(t1, t2) => write!(f, "({} {})", t1, t2),
            Term::Lit(literal) => write!(f, "{}", literal),
            Term::Cond(t1, t2, t3) => write!(f, "(if {} then {} else {})", t1, t2, t3),
            Term::Let(name, t1, t2) => write!(f, "(let {} = {} in {})", name.0, t1, t2),
            Term::Seq(t1, t2) => write!(f, "{} ; {}", t1, t2),
            Term::Fix(t1) => write!(f, "(fix {})", t1),
        }
    }
}

impl<'a> Term<'a> {
    pub fn from_ast(blk: Block<'a>) -> Self {
        lower_blk(blk)
    }
}

fn lower_blk(blk: Block<'_>) -> Term<'_> {
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

fn lower_node(node: Node<'_>) -> Term<'_> {
    match node {
        Node::Name(name) => Term::Var(name),
        Node::Cond(if_blk, do_blk, el_blk) => lower_cond(if_blk, do_blk, el_blk),
        Node::Literal(lit) => Term::Lit(lit),
        Node::Call(name, args) => lower_call(name, args),
        Node::BinaryOp(bin_op, node1, node2) => lower_binary_op(bin_op, *node1, *node2),
        Node::UnaryOp(un_op, node) => lower_unary_op(un_op, *node),
        Node::LetBind(name, node) => lower_let_bind(name, *node),
        Node::FnDef(name, binds, body, rec) => lower_fn_def(name, binds, body, rec),
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
    Term::BinaryOp(bin_op, Box::new(lower_node(node1)), Box::new(lower_node(node2)))
}

fn lower_unary_op(un_op: UnOp, node: Node<'_>) -> Term<'_> {
    Term::UnaryOp(un_op, Box::new(lower_node(node)))
}

fn lower_let_bind<'a>(name: Name<'a>, node: Node<'a>) -> Term<'a> {
    Term::Let(
        name,
        Box::new(lower_node(node)),
        Box::new(Term::Lit(Literal::Unit)),
    )
}

fn lower_fn_def<'a>(
    name: Name<'a>,
    binds: Vec<Binding<'a>>,
    body: Block<'a>,
    rec_ty: Option<Ty>,
) -> Term<'a> {
    let mut term = lower_blk(body);

    let ty = rec_ty.map(|mut ty| {
        for bind in binds.iter().rev() {
            ty = Ty::Arrow(Box::new(bind.ty.clone()), Box::new(ty));
        }
        ty
    });

    for bind in binds.into_iter().rev() {
        term = Term::Abs(bind, Box::new(term));
    }

    if let Some(ty) = ty {
        term = Term::Fix(Box::new(Term::Abs(Binding { name, ty }, Box::new(term))));
    }

    Term::Let(name, Box::new(term), Box::new(Term::Lit(Literal::Unit)))
}
