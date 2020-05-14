use std::include_str;

use pijama::ty::{Ty, TyError};
use pijama::{mir, parser, ty, LangError, LangResult};

fn type_check(input: &str) -> LangResult<Ty> {
    let ast = parser::parse(input)?;
    let mir = mir::Term::from_ast(ast);
    ty::ty_check(&mir)
}

#[test]
fn cond_ty_mismatch() {
    let input = include_str!("cond_ty_mismatch.pj");
    let error = type_check(input);
    assert!(matches!(
        error,
        Result::Err(LangError::Ty(TyError::Mismatch {
            expected: Ty::Bool,
            found: Ty::Int
        }))
    ));
}

#[test]
fn cond_res_ty_mismatch() {
    let input = include_str!("cond_res_ty_mismatch.pj");
    let error = type_check(input);
    assert!(matches!(
        error,
        Result::Err(LangError::Ty(TyError::Mismatch {
            expected: Ty::Bool,
            found: Ty::Int
        }))
    ));
}

#[test]
fn var_unbounded() {
    let input = include_str!("var_unbounded.pj");
    let error = type_check(input);
    assert!(matches!(
        error,
        Result::Err(LangError::Ty(TyError::Unbound(_)))
    ));
}
