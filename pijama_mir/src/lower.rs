use pijama_common::Primitive;
use pijama_ctx::{Context, ContextExt, LocalId, TermId, TypeInfo};
use pijama_hir::{BindKind as HirBindKind, Term as HirTerm, TermKind as HirTermKind};
use pijama_ty::Ty;

use crate::{BindKind, PrimFn, RValue, RValueKind, Term, TermKind};

pub(crate) fn lower_term(term: &HirTerm, ctx: &mut Context) -> Term {
    let kind = match &term.kind {
        HirTermKind::Lit(lit) => TermKind::Lit(*lit),
        HirTermKind::Var(local) => TermKind::Var(*local),
        HirTermKind::PrimFn(prim) => {
            let prim = match prim {
                Primitive::Print => PrimFn::Print,
            };
            TermKind::PrimApp(prim, vec![])
        }
        HirTermKind::UnaryOp(op, term) => {
            TermKind::PrimApp(PrimFn::UnOp(*op), vec![lower_term(term, ctx)])
        }
        HirTermKind::BinaryOp(op, t1, t2) => TermKind::PrimApp(
            PrimFn::BinOp(*op),
            vec![lower_term(t1, ctx), lower_term(t2, ctx)],
        ),
        HirTermKind::Abs(arg, body) => {
            let loc = ctx.get_location(term.id).unwrap();
            let info = ctx.get_type_info(term.id).unwrap().clone();

            let local_id: LocalId = ctx.new_id();
            ctx.insert_location(local_id, loc);
            ctx.insert_type_info(local_id, info.clone());

            let rvalue_id: TermId = ctx.new_id();
            ctx.insert_location(rvalue_id, loc);
            ctx.insert_type_info(rvalue_id, info.clone());

            let var_id: TermId = ctx.new_id();
            ctx.insert_location(var_id, loc);
            ctx.insert_type_info(var_id, info.clone());

            let rvalue = RValue {
                id: rvalue_id,
                kind: lower_abs(ctx, *arg, body.as_ref()),
            };

            TermKind::Let(
                BindKind::NonRec,
                local_id,
                Box::new(rvalue),
                Box::new(Term {
                    id: var_id,
                    kind: TermKind::Var(local_id),
                }),
            )
        }
        HirTermKind::App(func, arg) => {
            let mut func = func.as_ref();
            let mut args = vec![lower_term(arg, ctx)];

            while let HirTermKind::App(new_func, arg) = &func.kind {
                func = new_func;
                args.push(lower_term(arg, ctx));
            }

            args.reverse();

            let func_info = ctx.get_type_info(func.id).unwrap().clone();
            let count = args.len();

            let loc = ctx.get_location(func.id).unwrap();

            let mut new_args = vec![];
            let mut new_args_ty = func_info
                .ty
                .iter()
                .skip(count)
                .cloned()
                .collect::<Vec<Ty>>();

            let ret_ty = new_args_ty.pop().unwrap();

            for ty in &new_args_ty {
                let local = ctx.new_local();
                let local_id: LocalId = ctx.new_id();
                ctx.save_local(local_id, local);
                ctx.insert_location(local_id, loc);
                ctx.insert_type_info(
                    local_id,
                    TypeInfo {
                        ty: ty.clone(),
                        loc,
                    },
                );

                let term_id: TermId = ctx.new_id();
                ctx.insert_location(term_id, loc);
                ctx.insert_type_info(
                    term_id,
                    TypeInfo {
                        ty: ty.clone(),
                        loc,
                    },
                );

                new_args.push(local_id);
                args.push(Term {
                    id: term_id,
                    kind: TermKind::Var(local_id),
                });
            }

            let kind = if let HirTermKind::PrimFn(prim) = &func.kind {
                let prim = match prim {
                    Primitive::Print => PrimFn::Print,
                };
                TermKind::PrimApp(prim, args)
            } else {
                TermKind::App(Box::new(lower_term(func, ctx)), args)
            };

            if new_args.len() > 0 {
                let body_id: TermId = ctx.new_id();
                ctx.insert_location(body_id, loc);
                ctx.insert_type_info(body_id, TypeInfo { ty: ret_ty, loc });

                let local_id: LocalId = ctx.new_id();
                ctx.insert_location(local_id, loc);
                ctx.insert_type_info(local_id, func_info.clone());

                let rvalue_id: TermId = ctx.new_id();
                ctx.insert_location(rvalue_id, loc);
                ctx.insert_type_info(rvalue_id, func_info.clone());

                let var_id: TermId = ctx.new_id();
                ctx.insert_location(var_id, loc);
                ctx.insert_type_info(var_id, func_info.clone());

                let rvalue = RValue {
                    id: rvalue_id,
                    kind: RValueKind::Abs(new_args, Term { id: body_id, kind }),
                };

                TermKind::Let(
                    BindKind::NonRec,
                    local_id,
                    Box::new(rvalue),
                    Box::new(Term {
                        id: var_id,
                        kind: TermKind::Var(local_id),
                    }),
                )
            } else {
                kind
            }
        }
        HirTermKind::Cond(if_term, do_term, el_term) => TermKind::Cond(
            Box::new(lower_term(if_term, ctx)),
            Box::new(lower_term(do_term, ctx)),
            Box::new(lower_term(el_term, ctx)),
        ),
        HirTermKind::Let(kind, lhs, rhs, tail) => {
            let kind = match kind {
                HirBindKind::NonRec => BindKind::NonRec,
                HirBindKind::Rec => BindKind::Rec,
            };
            let rvalue = match &rhs.kind {
                HirTermKind::Abs(arg, body) => RValue {
                    id: rhs.id,
                    kind: lower_abs(ctx, *arg, body.as_ref()),
                },
                _ => lower_term(rhs.as_ref(), ctx).into_rvalue(),
            };
            TermKind::Let(
                kind,
                *lhs,
                Box::new(rvalue),
                Box::new(lower_term(tail.as_ref(), ctx)),
            )
        }
    };
    Term { id: term.id, kind }
}

fn lower_abs(ctx: &mut Context, arg: LocalId, mut body: &HirTerm) -> RValueKind {
    let mut args = vec![arg];

    while let HirTermKind::Abs(arg, new_body) = &body.kind {
        args.push(*arg);
        body = new_body;
    }

    RValueKind::Abs(args, lower_term(body, ctx))
}
