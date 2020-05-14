use std::include_str;

use pijama::ty::{Ty, TyError};
use pijama::{mir, parser, ty, LangError, LangResult};

fn type_check(input: &str) -> LangResult<Ty> {
    let ast = parser::parse(input)?;
    let mir = mir::Term::from_ast(ast);
    ty::ty_check(&mir)
}

#[test]
fn wrong_cond_input() {
    let input = include_str!("wrong_cond_input.pj");
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
fn wrong_cond_result() {
    let input = include_str!("wrong_cond_result.pj");
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
fn unary_minus_mismatch() {
    let input = include_str!("unary_minus_mismatch.pj");
    let error = type_check(input);
    assert!(matches!(
        error,
        Result::Err(LangError::Ty(TyError::Mismatch {
            expected: Ty::Int,
            found: Ty::Bool
        }))
    ));
}

#[test]
fn not_mismatch() {
    let input = include_str!("not_mismatch.pj");
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
fn mixed_type_binop_mismatch() {
    let input = include_str!("mixed_type_binop_mismatch.pj");
    let error = type_check(input);
    assert!(matches!(
        error,
        Result::Err(LangError::Ty(TyError::Mismatch {
            expected: Ty::Int,
            found: Ty::Bool
        }))
    ));
}

#[test]
fn wrong_type_int_binop_mismatch() {
    let input = include_str!("wrong_type_int_binop_mismatch.pj");
    let error = type_check(input);
    assert!(matches!(
        error,
        Result::Err(LangError::Ty(TyError::Mismatch {
            expected: Ty::Int,
            found: Ty::Bool
        }))
    ));
}

#[test]
fn wrong_type_bool_binop_mismatch() {
    let input = include_str!("wrong_type_bool_binop_mismatch.pj");
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
fn wrong_type_cmp_mismatch() {
    let input = include_str!("wrong_type_cmp_mismatch.pj");
    let error = type_check(input);
    assert!(matches!(
        error,
        Result::Err(LangError::Ty(TyError::Mismatch {
            expected: Ty::Int,
            found: Ty::Bool
        }))
    ));
}

#[test]
fn fn_arg_mismatch() {
    let input = include_str!("fn_arg_mismatch.pj");
    let error = type_check(input);
    assert!(matches!(
        error,
        Result::Err(LangError::Ty(TyError::Mismatch {
            expected: Ty::Int,
            found: Ty::Bool
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
