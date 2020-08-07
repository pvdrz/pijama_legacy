use pijama_common::{location::Located, Primitive};

use pijama_hir::{LetKind as HirLetKind, Term as HirTerm};

use crate::{LetKind, PrimFn, Term};

pub(crate) fn lower_term<'ast>(term: &Located<HirTerm<'ast>>) -> Term<'ast> {
    match &term.content {
        HirTerm::Lit(lit) => Term::Lit(*lit),
        HirTerm::Var(local) => Term::Var(*local),
        HirTerm::PrimFn(prim) => {
            let prim = match prim {
                Primitive::Print => PrimFn::Print,
            };
            Term::PrimApp(prim, vec![])
        }
        HirTerm::UnaryOp(op, term) => Term::PrimApp(PrimFn::UnOp(*op), vec![lower_term(term)]),
        HirTerm::BinaryOp(op, t1, t2) => {
            Term::PrimApp(PrimFn::BinOp(*op), vec![lower_term(t1), lower_term(t2)])
        }
        HirTerm::Abs(arg, _, body) => {
            let mut args = vec![*arg];
            let mut body = body.as_ref();

            while let HirTerm::Abs(arg, _, new_body) = &body.content {
                args.push(*arg);
                body = new_body;
            }

            Term::Abs(args, Box::new(lower_term(body)))
        }
        HirTerm::App(func, arg) => {
            let mut func = func.as_ref();
            let mut args = vec![lower_term(arg)];

            while let HirTerm::App(new_func, arg) = &func.content {
                func = new_func;
                args.push(lower_term(arg));
            }

            args.reverse();

            if let HirTerm::PrimFn(prim) = &func.content {
                let prim = match prim {
                    Primitive::Print => PrimFn::Print,
                };
                Term::PrimApp(prim, args)
            } else {
                Term::App(Box::new(lower_term(func)), args)
            }
        }
        HirTerm::Cond(if_term, do_term, el_term) => Term::Cond(
            Box::new(lower_term(if_term)),
            Box::new(lower_term(do_term)),
            Box::new(lower_term(el_term)),
        ),
        HirTerm::Let(kind, lhs, rhs, tail) => {
            let kind = match kind {
                HirLetKind::NonRec(_) => LetKind::NonRec,
                HirLetKind::Rec(_) => LetKind::Rec,
            };
            Term::Let(
                kind,
                lhs.content,
                Box::new(lower_term(rhs.as_ref())),
                Box::new(lower_term(tail.as_ref())),
            )
        }
    }
}
