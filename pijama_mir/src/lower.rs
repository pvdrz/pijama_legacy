use pijama_common::Primitive;
use pijama_hir::{BindKind as HirBindKind, Term as HirTerm, TermKind as HirTermKind};
use pijama_ty::context::{ContextExt, Context};

use crate::{LetKind, PrimFn, Term, TermKind};

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
            let mut args = vec![*arg];
            let mut body = body.as_ref();

            while let HirTermKind::Abs(arg, new_body) = &body.kind {
                args.push(*arg);
                body = new_body;
            }

            TermKind::Abs(args, Box::new(lower_term(body, ctx)))
        }
        HirTermKind::App(func, arg) => {
            let mut func = func.as_ref();
            let mut args = vec![lower_term(arg, ctx)];

            while let HirTermKind::App(new_func, arg) = &func.kind {
                func = new_func;
                args.push(lower_term(arg, ctx));
            }

            args.reverse();

            let arity = ctx.get_type_info(func.id).unwrap().ty.arity().unwrap();

            if let HirTermKind::PrimFn(prim) = &func.kind {
                let prim = match prim {
                    Primitive::Print => PrimFn::Print,
                };
                TermKind::PrimApp(prim, args)
            } else {
                TermKind::App(Box::new(lower_term(func, ctx)), args)
            }
        }
        HirTermKind::Cond(if_term, do_term, el_term) => TermKind::Cond(
            Box::new(lower_term(if_term, ctx)),
            Box::new(lower_term(do_term, ctx)),
            Box::new(lower_term(el_term, ctx)),
        ),
        HirTermKind::Let(kind, lhs, rhs, tail) => {
            let kind = match kind {
                HirBindKind::NonRec => LetKind::NonRec,
                HirBindKind::Rec => LetKind::Rec,
            };
            TermKind::Let(
                kind,
                *lhs,
                Box::new(lower_term(rhs.as_ref(), ctx)),
                Box::new(lower_term(tail.as_ref(), ctx)),
            )
        }
    };
    Term { id: term.id, kind }
}
