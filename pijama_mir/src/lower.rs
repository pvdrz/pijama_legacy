use thiserror::Error;

use std::mem::discriminant;

use pijama_ast::{
    analysis::is_fn_def_recursive,
    location::{Located, Location},
    node::{BinOp, Block, Branch, Expression, Name, Node, Statement, UnOp},
    ty::TyAnnotation,
};

use pijama_ty::Ty;

use crate::{LetKind, Term};

pub type LowerResult<T> = Result<T, LowerError>;

#[derive(Error, Debug)]
pub enum LowerError {
    #[error("Required type annotation is missing")]
    RequiredTy(Location),
    #[error("Anonymous functions cannot have a return type annotation")]
    AnonWithTy(Location),
}

impl LowerError {
    pub fn loc(&self) -> Location {
        match self {
            LowerError::RequiredTy(loc) | LowerError::AnonWithTy(loc) => *loc,
        }
    }
}

impl PartialEq for LowerError {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Eq for LowerError {}

pub fn lower_block<'a>(mut block: Block<'a>) -> LowerResult<Located<Term<'a>>> {
    if let Some(node) = block.nodes.pop_front() {
        match node {
            Node::Expr(expr) => {
                let head = lower_expression(expr)?;
                let tail = lower_block(block)?;
                let loc = head.loc + tail.loc;
                Ok(loc.with_content(Term::Seq(Box::new(head), Box::new(tail))))
            }
            Node::Stat(stat) => match stat.content {
                Statement::Assign(lhs, rhs) => lower_assign(stat.loc, lhs, rhs, block),
                Statement::FnDef(name, args, body) => {
                    lower_fn_def(stat.loc, name, args, body, block)
                }
            },
        }
    } else {
        lower_expression(*block.expr)
    }
}

fn lower_expression(expr: Located<Expression<'_>>) -> LowerResult<Located<Term<'_>>> {
    let loc = expr.loc;
    match expr.content {
        Expression::Name(name) => Ok(loc.with_content(Term::Var(name))),
        Expression::Literal(lit) => Ok(loc.with_content(Term::Lit(lit))),
        Expression::PrimFn(prim) => Ok(loc.with_content(Term::PrimFn(prim))),
        Expression::Cond(if_branch, branches, el_blk) => {
            lower_cond(loc, if_branch, branches, el_blk)
        }
        Expression::Call(func, args) => lower_call(loc, *func, args),
        Expression::BinaryOp(bin_op, expr1, expr2) => lower_binary_op(loc, bin_op, *expr1, *expr2),
        Expression::UnaryOp(un_op, expr) => lower_unary_op(loc, un_op, *expr),
        Expression::AnonFn(args, body) => lower_anon_fn(loc, args, body),
    }
}

fn lower_cond<'a>(
    loc: Location,
    if_branch: Branch<'a>,
    branches: Vec<Branch<'a>>,
    el_blk: Block<'a>,
) -> LowerResult<Located<Term<'a>>> {
    let mut el_term = Box::new(lower_block(el_blk)?);

    for branch in branches.into_iter().rev() {
        el_term = Box::new(loc.with_content(Term::Cond(
            Box::new(lower_block(branch.cond)?),
            Box::new(lower_block(branch.body)?),
            el_term,
        )));
    }

    let if_blk = if_branch.cond;
    let do_blk = if_branch.body;

    Ok(loc.with_content(Term::Cond(
        Box::new(lower_block(if_blk)?),
        Box::new(lower_block(do_blk)?),
        el_term,
    )))
}

fn lower_call<'a>(
    loc: Location,
    func: Located<Expression<'a>>,
    args: Vec<Located<Expression<'a>>>,
) -> LowerResult<Located<Term<'a>>> {
    let mut term = lower_expression(func)?;
    for arg in args {
        term = loc.with_content(Term::App(Box::new(term), Box::new(lower_expression(arg)?)));
    }
    Ok(term)
}

fn lower_binary_op<'a>(
    loc: Location,
    bin_op: BinOp,
    expr1: Located<Expression<'a>>,
    expr2: Located<Expression<'a>>,
) -> LowerResult<Located<Term<'a>>> {
    Ok(loc.with_content(Term::BinaryOp(
        bin_op,
        Box::new(lower_expression(expr1)?),
        Box::new(lower_expression(expr2)?),
    )))
}

fn lower_unary_op(
    loc: Location,
    un_op: UnOp,
    expr: Located<Expression<'_>>,
) -> LowerResult<Located<Term<'_>>> {
    Ok(loc.with_content(Term::UnaryOp(un_op, Box::new(lower_expression(expr)?))))
}

fn lower_assign<'a>(
    loc: Location,
    lhs: TyAnnotation<Located<Name<'a>>>,
    rhs: Located<Expression<'a>>,
    tail: Block<'a>,
) -> LowerResult<Located<Term<'a>>> {
    let rhs = lower_expression(rhs)?;

    let opt_ty = if let Some(ty) = Ty::from_ast(lhs.ty.content) {
        Some(lhs.ty.loc.with_content(ty))
    } else {
        None
    };

    let tail = lower_block(tail)?;

    Ok(loc.with_content(Term::Let(
        LetKind::NonRec(opt_ty),
        lhs.item,
        Box::new(rhs),
        Box::new(tail),
    )))
}

fn lower_fn_def<'a>(
    loc: Location,
    name: Located<Name<'a>>,
    args: Vec<TyAnnotation<Located<Name<'a>>>>,
    body: TyAnnotation<Block<'a>>,
    tail: Block<'a>,
) -> LowerResult<Located<Term<'a>>> {
    // if the user added a return type annotation, we transform this type into the type of the
    // function using the arguments' annotations.
    let ty_loc = body.ty.loc;
    let opt_ty = if let Some(mut ty) = Ty::from_ast(body.ty.content) {
        for arg in args.iter().rev() {
            let arg_ty = Ty::from_ast(arg.ty.content.clone())
                .ok_or_else(|| LowerError::RequiredTy(arg.ty.loc))?;
            ty = Ty::Arrow(Box::new(arg_ty), Box::new(ty));
        }
        Some(ty_loc.with_content(ty))
    } else {
        None
    };

    // we need to decide if the function is recursive or not
    let kind = if is_fn_def_recursive(name.content, &body.item) {
        // if the function is recursive, we need the return type.
        opt_ty
            .map(LetKind::Rec)
            .ok_or_else(|| LowerError::RequiredTy(name.loc))?
    } else {
        LetKind::NonRec(opt_ty)
    };

    let mut term = lower_block(body.item)?;

    for arg in args.into_iter().rev() {
        let loc = arg.ty.loc;
        term = loc.with_content(Term::Abs(
            arg.item.content,
            Ty::from_ast(arg.ty.content).ok_or_else(|| LowerError::RequiredTy(loc))?,
            Box::new(term),
        ));
    }

    let tail = lower_block(tail)?;

    term = loc.with_content(Term::Let(kind, name, Box::new(term), Box::new(tail)));

    Ok(term)
}

fn lower_anon_fn<'a>(
    loc: Location,
    args: Vec<TyAnnotation<Located<Name<'a>>>>,
    body: TyAnnotation<Block<'a>>,
) -> LowerResult<Located<Term<'a>>> {
    if Ty::from_ast(body.ty.content).is_some() {
        return Err(LowerError::AnonWithTy(body.ty.loc));
    }

    let mut term = lower_block(body.item)?;

    for arg in args.into_iter().rev() {
        term = loc.with_content(Term::Abs(
            arg.item.content,
            Ty::from_ast(arg.ty.content).unwrap(),
            Box::new(term),
        ));
    }

    Ok(term)
}
