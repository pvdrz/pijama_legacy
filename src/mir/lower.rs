use std::mem::discriminant;

use pijama_ast::{
    analysis::RecursionChecker, ty::TyAnnotation, BinOp, Block, Branch, Literal, Located, Location,
    Name, Node, UnOp,
};
use thiserror::Error;

use crate::{
    mir::{LetKind, Term},
    ty::Ty,
};

pub type LowerResult<T> = Result<T, LowerError>;

#[derive(Error, Debug)]
pub enum LowerError {
    #[error("Recursive functions need a return type annotation")]
    RecWithoutTy(Location),
    #[error("Anonymous functions cannot have a return type annotation")]
    AnonWithTy(Location),
}

impl LowerError {
    pub fn loc(&self) -> Location {
        match self {
            LowerError::RecWithoutTy(loc) | LowerError::AnonWithTy(loc) => *loc,
        }
    }
}

impl PartialEq for LowerError {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Eq for LowerError {}

pub fn lower_blk<'a>(blk: Located<Block<'a>>) -> LowerResult<Located<Term<'a>>> {
    let mut terms = blk.content.into_iter().rev().map(lower_node);
    if let Some(term) = terms.next() {
        let mut term = term?;
        for prev_term in terms {
            let prev_term = prev_term?;
            let next_term = Box::new(term);

            let loc = prev_term.loc;
            let content = if let Term::Let(kind, name, value, _) = prev_term.content {
                Term::Let(kind, name, value, next_term)
            } else {
                Term::Seq(Box::new(prev_term), next_term)
            };
            term = loc.with_content(content);
        }
        Ok(term)
    } else {
        Ok(blk.loc.with_content(Term::Lit(Literal::Unit)))
    }
}

fn lower_node(node: Located<Node<'_>>) -> LowerResult<Located<Term<'_>>> {
    let loc = node.loc;
    match node.content {
        Node::Name(name) => Ok(loc.with_content(Term::Var(name))),
        Node::Literal(lit) => Ok(loc.with_content(Term::Lit(lit))),
        Node::PrimFn(prim) => Ok(loc.with_content(Term::PrimFn(prim))),
        Node::Cond(if_branch, branches, el_blk) => lower_cond(loc, if_branch, branches, el_blk),
        Node::Call(node, args) => lower_call(loc, *node, args),
        Node::BinaryOp(bin_op, node1, node2) => lower_binary_op(loc, bin_op, *node1, *node2),
        Node::UnaryOp(un_op, node) => lower_unary_op(loc, un_op, *node),
        Node::AnonFn(binds, body) => lower_anon_fn(loc, binds, body),
        node @ Node::LetBind(_, _) | node @ Node::FnDef(_, _, _) => {
            let empty_blk = Location::new(loc.end, loc.end).with_content(Block::default());
            match node {
                Node::LetBind(annotation, body) => {
                    lower_let_bind(loc, annotation, *body, empty_blk)
                }
                Node::FnDef(name, binds, body) => lower_fn_def(loc, name, binds, body, empty_blk),
                _ => unreachable!(),
            }
        }
    }
}

fn lower_cond<'a>(
    loc: Location,
    if_branch: Branch<'a>,
    branches: Vec<Branch<'a>>,
    el_blk: Located<Block<'a>>,
) -> LowerResult<Located<Term<'a>>> {
    let mut el_term = Box::new(lower_blk(el_blk)?);

    for branch in branches.into_iter().rev() {
        el_term = Box::new(loc.with_content(Term::Cond(
            Box::new(lower_blk(branch.cond)?),
            Box::new(lower_blk(branch.body)?),
            el_term,
        )));
    }

    let if_blk = if_branch.cond;
    let do_blk = if_branch.body;

    Ok(loc.with_content(Term::Cond(
        Box::new(lower_blk(if_blk)?),
        Box::new(lower_blk(do_blk)?),
        el_term,
    )))
}

fn lower_call<'a>(
    loc: Location,
    node: Located<Node<'a>>,
    args: Block<'a>,
) -> LowerResult<Located<Term<'a>>> {
    let mut term = lower_node(node)?;
    for node in args {
        term = loc.with_content(Term::App(Box::new(term), Box::new(lower_node(node)?)));
    }
    Ok(term)
}

fn lower_binary_op<'a>(
    loc: Location,
    bin_op: BinOp,
    node1: Located<Node<'a>>,
    node2: Located<Node<'a>>,
) -> LowerResult<Located<Term<'a>>> {
    Ok(loc.with_content(Term::BinaryOp(
        bin_op,
        Box::new(lower_node(node1)?),
        Box::new(lower_node(node2)?),
    )))
}

fn lower_unary_op(
    loc: Location,
    un_op: UnOp,
    node: Located<Node<'_>>,
) -> LowerResult<Located<Term<'_>>> {
    Ok(loc.with_content(Term::UnaryOp(un_op, Box::new(lower_node(node)?))))
}

fn lower_let_bind<'a>(
    loc: Location,
    annotation: TyAnnotation<Name<'a>>,
    body: Located<Node<'a>>,
    tail: Located<Block<'a>>,
) -> LowerResult<Located<Term<'a>>> {
    let body = lower_node(body)?;

    let opt_ty = if let Some(ty) = Ty::from_ast(annotation.ty.content) {
        Some(annotation.ty.loc.with_content(ty))
    } else {
        None
    };

    let tail = lower_blk(tail)?;

    Ok(loc.with_content(Term::Let(
        LetKind::NonRec(opt_ty),
        annotation.item,
        Box::new(body),
        Box::new(tail),
    )))
}

fn lower_fn_def<'a>(
    loc: Location,
    name: Located<Name<'a>>,
    annotations: Vec<TyAnnotation<Name<'a>>>,
    body: TyAnnotation<Block<'a>>,
    tail: Located<Block<'a>>,
) -> LowerResult<Located<Term<'a>>> {
    // if the user added a return type annotation, we transform this type into the type of the
    // function using the bindings.
    let ty_loc = body.ty.loc;
    let opt_ty = if let Some(mut ty) = Ty::from_ast(body.ty.content) {
        for annotation in annotations.iter().rev() {
            // FIXME: There could be missing types here!
            let ann_ty = Ty::from_ast(annotation.ty.content.clone()).unwrap();
            ty = Ty::Arrow(Box::new(ann_ty), Box::new(ty));
        }
        Some(ty_loc.with_content(ty))
    } else {
        None
    };

    // we need to decide if the function is recursive or not
    let kind = if RecursionChecker::run(name.content, &body.item.content) {
        // if the function is recursive, we need the return type.
        opt_ty
            .map(LetKind::Rec)
            .ok_or_else(|| LowerError::RecWithoutTy(name.loc))?
    } else {
        LetKind::NonRec(opt_ty)
    };

    let mut term = lower_blk(body.item)?;

    for annotation in annotations.into_iter().rev() {
        term = loc.with_content(Term::Abs(
            annotation.item.content,
            Ty::from_ast(annotation.ty.content).unwrap(),
            Box::new(term),
        ));
    }

    let tail = lower_blk(tail)?;

    term = loc.with_content(Term::Let(kind, name, Box::new(term), Box::new(tail)));

    Ok(term)
}

fn lower_anon_fn<'a>(
    loc: Location,
    annotations: Vec<TyAnnotation<Name<'a>>>,
    body: TyAnnotation<Block<'a>>,
) -> LowerResult<Located<Term<'a>>> {
    if let Some(_) = Ty::from_ast(body.ty.content) {
        return Err(LowerError::AnonWithTy(body.ty.loc));
    }

    let mut term = lower_blk(body.item)?;

    for annotation in annotations.into_iter().rev() {
        term = loc.with_content(Term::Abs(
            annotation.item.content,
            Ty::from_ast(annotation.ty.content).unwrap(),
            Box::new(term),
        ));
    }

    Ok(term)
}
