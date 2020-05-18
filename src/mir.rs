use std::fmt;

use crate::ast::*;
use crate::ty::{expect_ty, ty_check, Binding, Ty, TyResult};
use crate::{LangError, LangResult};

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
    pub fn from_ast(blk: Block<'a>) -> LangResult<Self> {
        lower_blk(blk).map_err(LangError::Ty)
    }
}

fn lower_blk<'a>(blk: Block<'a>) -> TyResult<Term<'a>> {
    let mut terms = blk.into_iter().rev().map(lower_node);
    if let Some(term) = terms.next() {
        let mut term = term?;
        for prev_term in terms {
            let prev_term = prev_term?;
            let next_term = Box::new(term);
            term = if let Term::Let(name, value, _) = prev_term {
                Term::Let(name, value, next_term)
            } else {
                Term::Seq(Box::new(prev_term), next_term)
            };
        }
        Ok(term)
    } else {
        Ok(Term::Lit(Literal::Unit))
    }
}

fn lower_node(node: Node<'_>) -> TyResult<Term<'_>> {
    match node {
        Node::Name(name) => Ok(Term::Var(name)),
        Node::Cond(if_blk, do_blk, el_blk) => lower_cond(if_blk, do_blk, el_blk),
        Node::Literal(lit) => Ok(Term::Lit(lit)),
        Node::Call(name, args) => lower_call(name, args),
        Node::BinaryOp(bin_op, node1, node2) => lower_binary_op(bin_op, *node1, *node2),
        Node::UnaryOp(un_op, node) => lower_unary_op(un_op, *node),
        Node::LetBind(name, opt_ty, node) => lower_let_bind(name, opt_ty, *node),
        Node::FnDef(opt_name, binds, body, opt_ty) => lower_fn_def(opt_name, binds, body, opt_ty),
        Node::FnRecDef(name, binds, body, ty) => lower_fn_rec_def(name, binds, body, ty),
    }
}

fn lower_cond<'a>(if_blk: Block<'a>, do_blk: Block<'a>, el_blk: Block<'a>) -> TyResult<Term<'a>> {
    Ok(Term::Cond(
        Box::new(lower_blk(if_blk)?),
        Box::new(lower_blk(do_blk)?),
        Box::new(lower_blk(el_blk)?),
    ))
}

fn lower_call<'a>(name: Name<'a>, args: Block<'a>) -> TyResult<Term<'a>> {
    let mut term = Term::Var(name);
    for node in args {
        term = Term::App(Box::new(term), Box::new(lower_node(node)?));
    }

    Ok(term)
}

fn lower_binary_op<'a>(bin_op: BinOp, node1: Node<'a>, node2: Node<'a>) -> TyResult<Term<'a>> {
    Ok(Term::BinaryOp(
        bin_op,
        Box::new(lower_node(node1)?),
        Box::new(lower_node(node2)?),
    ))
}

fn lower_unary_op(un_op: UnOp, node: Node<'_>) -> TyResult<Term<'_>> {
    Ok(Term::UnaryOp(un_op, Box::new(lower_node(node)?)))
}

fn lower_let_bind<'a>(name: Name<'a>, opt_ty: Option<Ty>, node: Node<'a>) -> TyResult<Term<'a>> {
    let term = lower_node(node)?;

    if let Some(ty) = opt_ty {
        let term_ty = ty_check(&term)?;
        expect_ty(ty, term_ty)?;
    }

    Ok(Term::Let(
        name,
        Box::new(term),
        Box::new(Term::Lit(Literal::Unit)),
    ))
}

fn lower_fn_def<'a>(
    opt_name: Option<Name<'a>>,
    binds: Vec<Binding<'a>>,
    body: Block<'a>,
    opt_ty: Option<Ty>,
) -> TyResult<Term<'a>> {
    let mut term = lower_blk(body)?;

    let opt_ty = opt_ty.map(|mut ty| {
        for bind in binds.iter().rev() {
            ty = Ty::Arrow(Box::new(bind.ty.clone()), Box::new(ty));
        }
        ty
    });

    for bind in binds.into_iter().rev() {
        term = Term::Abs(bind, Box::new(term));
    }

    if let Some(ty) = opt_ty {
        let term_ty = ty_check(&term)?;
        expect_ty(ty, term_ty)?;
    }

    if let Some(name) = opt_name {
        term = Term::Let(name, Box::new(term), Box::new(Term::Lit(Literal::Unit)));
    }

    Ok(term)
}

fn lower_fn_rec_def<'a>(
    name: Name<'a>,
    binds: Vec<Binding<'a>>,
    body: Block<'a>,
    mut ty: Ty,
) -> TyResult<Term<'a>> {
    let mut term = lower_blk(body)?;

    for bind in binds.into_iter().rev() {
        ty = Ty::Arrow(Box::new(bind.ty.clone()), Box::new(ty));
        term = Term::Abs(bind, Box::new(term));
    }

    term = Term::Fix(Box::new(Term::Abs(Binding { name, ty }, Box::new(term))));

    Ok(Term::Let(
        name,
        Box::new(term),
        Box::new(Term::Lit(Literal::Unit)),
    ))
}
