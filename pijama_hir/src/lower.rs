use thiserror::Error;

use pijama_ast::{
    analysis::is_fn_def_recursive,
    node::{Block, Branch, Expression, Node, Statement},
    ty::{Ty as AstTy, TyAnnotation},
};
use pijama_common::{
    location::{Located, LocatedError, Location},
    BinOp, Local, UnOp,
};
use pijama_ctx::{Context, ContextExt, LocalId, TermId, TypeInfo};
use pijama_ty::Ty;

use crate::{BindKind, Term, TermKind};

pub fn lower_ast<'ast, 'ctx>(
    ctx: &'ctx mut Context<'ast>,
    block: Block<'ast>,
) -> LowerResult<Term> {
    let mut scope = Scope::new(ctx);
    let term = scope.lower_block(block)?;
    Ok(term)
}

pub type LowerResult<T> = Result<T, LowerError>;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum LowerErrorKind {
    #[error("Required type annotation is missing")]
    RequiredTy,
    #[error("Anonymous functions cannot have a return type annotation")]
    AnonWithTy,
    #[error("Local {0} is not bounded in the current scope")]
    Unbounded(String),
}

pub type LowerError = LocatedError<LowerErrorKind>;

fn require_ty(ty: &AstTy, loc: Location) -> LowerResult<()> {
    if let AstTy::Missing = ty {
        Err(LowerError::new(LowerErrorKind::RequiredTy, loc))
    } else {
        Ok(())
    }
}

struct Scope<'ast, 'ctx> {
    ctx: &'ctx mut Context<'ast>,
    locals: Vec<(Local<'ast>, LocalId)>,
}

impl<'ast, 'ctx> Scope<'ast, 'ctx> {
    fn new(ctx: &'ctx mut Context<'ast>) -> Self {
        Self {
            ctx,
            locals: vec![],
        }
    }

    fn lower_ty(&mut self, ty: AstTy) -> Ty {
        match ty {
            AstTy::Bool => Ty::Bool,
            AstTy::Int => Ty::Int,
            AstTy::Unit => Ty::Unit,
            AstTy::Missing => self.ctx.new_ty(),
            AstTy::Arrow(ty1, ty2) => {
                Ty::Arrow(Box::new(self.lower_ty(*ty1)), Box::new(self.lower_ty(*ty2)))
            }
        }
    }

    fn push_local(&mut self, local: TyAnnotation<Located<Local<'ast>>>) -> LocalId {
        let ty = self.lower_ty(local.ty.content);
        let ty_loc = local.ty.loc;
        let loc = local.item.loc;
        let local = local.item.content;

        let id: LocalId = self.ctx.new_id();
        self.ctx.save_local(id, local);

        self.ctx.insert_location(id, loc);
        self.ctx.insert_type_info(id, TypeInfo { ty, loc: ty_loc });

        self.locals.push((local, id));

        id
    }

    fn pop_local(&mut self) -> (Local<'ast>, LocalId) {
        self.locals.pop().expect("Stack of locals is empty")
    }

    pub fn lower_block(&mut self, mut block: Block<'ast>) -> LowerResult<Term> {
        if let Some(node) = block.nodes.pop_front() {
            match node {
                Node::Expr(expr) => {
                    let head_loc = expr.loc;
                    let head = self.lower_expression(expr)?;
                    let tail = self.lower_block(block)?;

                    let loc = head_loc + self.ctx.get_location(tail.id).unwrap();

                    let term_id: TermId = self.ctx.new_id();
                    self.ctx.insert_location(term_id, loc);

                    let local_id: LocalId = self.ctx.new_id();
                    let local_ty = self.ctx.new_ty();
                    self.ctx.save_local(local_id, Local::Wildcard);
                    self.ctx.insert_location(local_id, head_loc);
                    self.ctx.insert_type_info(
                        local_id,
                        TypeInfo {
                            ty: local_ty,
                            loc: head_loc,
                        },
                    );
                    Ok(Term::new(
                        term_id,
                        TermKind::Let(BindKind::NonRec, local_id, Box::new(head), Box::new(tail)),
                    ))
                }
                Node::Stat(stat) => match stat.content {
                    Statement::Assign(lhs, rhs) => self.lower_assign(stat.loc, lhs, rhs, block),
                    Statement::FnDef(name, args, body) => {
                        self.lower_fn_def(stat.loc, name, args, body, block)
                    }
                },
            }
        } else {
            self.lower_expression(*block.expr)
        }
    }

    fn lower_expression(&mut self, expr: Located<Expression<'ast>>) -> LowerResult<Term> {
        let loc = expr.loc;
        match expr.content {
            Expression::Local(local) => {
                for &(local2, local_id) in self.locals.iter().rev() {
                    if local == local2 {
                        let term_id: TermId = self.ctx.new_id();
                        self.ctx.insert_location(term_id, loc);
                        return Ok(Term::new(term_id, TermKind::Var(local_id)));
                    }
                }
                Err(LowerError::new(
                    LowerErrorKind::Unbounded(local.to_string()),
                    loc,
                ))
            }
            Expression::Literal(lit) => {
                let term_id: TermId = self.ctx.new_id();
                self.ctx.insert_location(term_id, loc);
                Ok(Term::new(term_id, TermKind::Lit(lit)))
            }
            Expression::PrimFn(prim) => {
                let term_id: TermId = self.ctx.new_id();
                self.ctx.insert_location(term_id, loc);
                Ok(Term::new(term_id, TermKind::PrimFn(prim)))
            }
            Expression::Cond(if_branch, branches, else_block) => {
                self.lower_cond(loc, if_branch, branches, else_block)
            }
            Expression::Call(func, args) => self.lower_call(loc, *func, args),
            Expression::BinaryOp(bin_op, expr1, expr2) => {
                self.lower_binary_op(loc, bin_op, *expr1, *expr2)
            }
            Expression::UnaryOp(un_op, expr) => self.lower_unary_op(loc, un_op, *expr),
            Expression::AnonFn(args, body) => self.lower_anon_fn(loc, args, body),
        }
    }

    fn lower_cond(
        &mut self,
        loc: Location,
        if_branch: Branch<'ast>,
        branches: Vec<Branch<'ast>>,
        else_block: Block<'ast>,
    ) -> LowerResult<Term> {
        let term_id: TermId = self.ctx.new_id();
        self.ctx.insert_location(term_id, loc);

        let mut else_term = self.lower_block(else_block)?;

        for branch in branches.into_iter().rev() {
            let term_id: TermId = self.ctx.new_id();
            self.ctx.insert_location(term_id, loc);

            else_term = Term::new(
                term_id,
                TermKind::Cond(
                    Box::new(self.lower_block(branch.cond)?),
                    Box::new(self.lower_block(branch.body)?),
                    Box::new(else_term),
                ),
            );
        }

        let if_block = if_branch.cond;
        let do_block = if_branch.body;

        Ok(Term::new(
            term_id,
            TermKind::Cond(
                Box::new(self.lower_block(if_block)?),
                Box::new(self.lower_block(do_block)?),
                Box::new(else_term),
            ),
        ))
    }

    fn lower_call(
        &mut self,
        loc: Location,
        func: Located<Expression<'ast>>,
        args: Vec<Located<Expression<'ast>>>,
    ) -> LowerResult<Term> {
        let term_id: TermId = self.ctx.new_id();
        self.ctx.insert_location(term_id, loc);

        let mut term = self.lower_expression(func)?;
        for arg in args {
            let term_id = self.ctx.new_id();
            self.ctx.insert_location(term_id, loc);
            term = Term::new(
                term_id,
                TermKind::App(Box::new(term), Box::new(self.lower_expression(arg)?)),
            );
        }

        Ok(term)
    }

    fn lower_binary_op(
        &mut self,
        loc: Location,
        bin_op: BinOp,
        expr1: Located<Expression<'ast>>,
        expr2: Located<Expression<'ast>>,
    ) -> LowerResult<Term> {
        let term_id: TermId = self.ctx.new_id();
        self.ctx.insert_location(term_id, loc);

        Ok(Term::new(
            term_id,
            TermKind::BinaryOp(
                bin_op,
                Box::new(self.lower_expression(expr1)?),
                Box::new(self.lower_expression(expr2)?),
            ),
        ))
    }

    fn lower_unary_op(
        &mut self,
        loc: Location,
        un_op: UnOp,
        expr: Located<Expression<'ast>>,
    ) -> LowerResult<Term> {
        let term_id: TermId = self.ctx.new_id();
        self.ctx.insert_location(term_id, loc);

        Ok(Term::new(
            term_id,
            TermKind::UnaryOp(un_op, Box::new(self.lower_expression(expr)?)),
        ))
    }

    fn lower_assign(
        &mut self,
        loc: Location,
        lhs: TyAnnotation<Located<Local<'ast>>>,
        rhs: Located<Expression<'ast>>,
        tail: Block<'ast>,
    ) -> LowerResult<Term> {
        let term_id: TermId = self.ctx.new_id();
        self.ctx.insert_location(term_id, loc);

        let rhs = self.lower_expression(rhs)?;

        let lhs_id = self.push_local(lhs);

        let tail = self.lower_block(tail)?;

        self.pop_local();

        Ok(Term::new(
            term_id,
            TermKind::Let(BindKind::NonRec, lhs_id, Box::new(rhs), Box::new(tail)),
        ))
    }

    fn lower_fn_def(
        &mut self,
        loc: Location,
        name: Located<Local<'ast>>,
        args: Vec<TyAnnotation<Located<Local<'ast>>>>,
        body: TyAnnotation<Block<'ast>>,
        tail: Block<'ast>,
    ) -> LowerResult<Term> {
        let term_id: TermId = self.ctx.new_id();
        self.ctx.insert_location(term_id, loc);

        let is_recursive = is_fn_def_recursive(name.content, &body.item);

        let arity = args.len();

        let body_ty = body.ty.content;

        // we need to decide if the function is recursive or not
        let kind = if is_recursive {
            // if the function is recursive, we need the return type.
            // FIXME this restriction is artificial now.
            require_ty(&body_ty, loc)?;

            let loc = name.loc;
            let local = name.content;
            let id: LocalId = self.ctx.new_id();
            self.ctx.save_local(id, local);
            self.ctx.insert_location(id, loc);

            self.locals.push((local, id));

            BindKind::Rec
        } else {
            BindKind::NonRec
        };

        for arg in args {
            self.push_local(arg);
        }

        let mut term = self.lower_block(body.item)?;

        let mut term_ty = self.lower_ty(body_ty);
        self.ctx.insert_type_info(
            term.id,
            TypeInfo {
                ty: term_ty.clone(),
                loc: body.ty.loc,
            },
        );

        for _ in 0..arity {
            let (_, arg_id) = self.pop_local();

            let mut term_info = self.ctx.get_type_info(arg_id).unwrap().clone();
            term_ty = Ty::Arrow(Box::new(term_info.ty), Box::new(term_ty));
            term_info.ty = term_ty.clone();

            let term_id: TermId = self.ctx.new_id();
            self.ctx.insert_location(term_id, loc);
            self.ctx.insert_type_info(term_id, term_info);

            term = Term::new(term_id, TermKind::Abs(arg_id, Box::new(term)));
        }

        if !is_recursive {
            let loc = name.loc;
            let local = name.content;
            let id: LocalId = self.ctx.new_id();
            self.ctx.save_local(id, local);
            self.ctx.insert_location(id, loc);

            self.locals.push((local, id));
        }

        let tail = self.lower_block(tail)?;

        let (_, local_id) = self.pop_local();

        self.ctx.insert_type_info(
            local_id,
            TypeInfo {
                ty: term_ty,
                loc: body.ty.loc,
            },
        );

        term = Term::new(
            term_id,
            TermKind::Let(kind, local_id, Box::new(term), Box::new(tail)),
        );

        Ok(term)
    }

    fn lower_anon_fn(
        &mut self,
        loc: Location,
        args: Vec<TyAnnotation<Located<Local<'ast>>>>,
        body: TyAnnotation<Block<'ast>>,
    ) -> LowerResult<Term> {
        // FIXME this restriction is artificial now.
        if let AstTy::Missing = body.ty.content {
            ()
        } else {
            return Err(LowerError::new(LowerErrorKind::AnonWithTy, body.ty.loc));
        };

        let arity = args.len();

        for arg in args {
            self.push_local(arg);
        }

        let mut term = self.lower_block(body.item)?;

        let mut term_ty = self.ctx.new_ty();
        self.ctx.insert_type_info(
            term.id,
            TypeInfo {
                ty: term_ty.clone(),
                loc: body.ty.loc,
            },
        );

        for _ in 0..arity {
            let (_, arg_id) = self.pop_local();

            let mut term_info = self.ctx.get_type_info(arg_id).unwrap().clone();
            term_ty = Ty::Arrow(Box::new(term_info.ty), Box::new(term_ty));
            term_info.ty = term_ty.clone();

            let term_id: TermId = self.ctx.new_id();
            self.ctx.insert_location(term_id, loc);
            self.ctx.insert_type_info(term_id, term_info);

            term = Term::new(term_id, TermKind::Abs(arg_id, Box::new(term)));
        }

        Ok(term)
    }
}
