use std::include_str;

use super::type_check;
use pijama::ty::{Ty, TyError};
use pijama::LangError;

#[test]
fn wrong_cond_input_mismatch() {
    let input = include_str!("wrong_cond_input_mismatch.pj");
    let error = type_check(input);
    assert!(
        matches!(
            error,
            Result::Err(LangError::Ty(TyError::Mismatch {
                expected: Ty::Bool,
                found: Ty::Int
            }))
        ),
        "{:#?}",
        error
    );
}

#[test]
fn wrong_cond_result_mismatch() {
    let input = include_str!("wrong_cond_result_mismatch.pj");
    let error = type_check(input);
    assert!(
        matches!(
            error,
            Result::Err(LangError::Ty(TyError::Mismatch {
                expected: Ty::Bool,
                found: Ty::Int
            }))
        ),
        "{:#?}",
        error
    );
}

#[test]
fn unary_minus_mismatch() {
    let input = include_str!("unary_minus_mismatch.pj");
    let error = type_check(input);
    assert!(
        matches!(
            error,
            Result::Err(LangError::Ty(TyError::Mismatch {
                expected: Ty::Int,
                found: Ty::Bool
            }))
        ),
        "{:#?}",
        error
    );
}

#[test]
fn not_mismatch() {
    let input = include_str!("not_mismatch.pj");
    let error = type_check(input);
    assert!(
        matches!(
            error,
            Result::Err(LangError::Ty(TyError::Mismatch {
                expected: Ty::Bool,
                found: Ty::Int
            }))
        ),
        "{:#?}",
        error
    );
}

#[test]
fn mixed_type_binop_mismatch() {
    let input = include_str!("mixed_type_binop_mismatch.pj");
    let error = type_check(input);
    assert!(
        matches!(
            error,
            Result::Err(LangError::Ty(TyError::Mismatch {
                expected: Ty::Int,
                found: Ty::Bool
            }))
        ),
        "{:#?}",
        error
    );
}

#[test]
fn wrong_type_int_binop_mismatch() {
    let input = include_str!("wrong_type_int_binop_mismatch.pj");
    let error = type_check(input);
    assert!(
        matches!(
            error,
            Result::Err(LangError::Ty(TyError::Mismatch {
                expected: Ty::Int,
                found: Ty::Bool
            }))
        ),
        "{:#?}",
        error
    );
}

#[test]
fn wrong_type_bool_binop_mismatch() {
    let input = include_str!("wrong_type_bool_binop_mismatch.pj");
    let error = type_check(input);
    assert!(
        matches!(
            error,
            Result::Err(LangError::Ty(TyError::Mismatch {
                expected: Ty::Bool,
                found: Ty::Int
            }))
        ),
        "{:#?}",
        error
    );
}

#[test]
fn wrong_type_cmp_mismatch() {
    let input = include_str!("wrong_type_cmp_mismatch.pj");
    let error = type_check(input);
    assert!(
        matches!(
            error,
            Result::Err(LangError::Ty(TyError::Mismatch {
                expected: Ty::Int,
                found: Ty::Bool
            }))
        ),
        "{:#?}",
        error
    );
}

#[test]
fn fn_arg_mismatch() {
    let input = include_str!("fn_arg_mismatch.pj");
    let error = type_check(input);
    assert!(
        matches!(
            error,
            Result::Err(LangError::Ty(TyError::Mismatch {
                expected: Ty::Int,
                found: Ty::Bool
            }))
        ),
        "{:#?}",
        error
    );
}

#[test]
fn var_unbounded() {
    let input = include_str!("var_unbounded.pj");
    let error = type_check(input);
    assert!(
        matches!(error, Result::Err(LangError::Ty(TyError::Unbound(_)))),
        "{:#?}",
        error
    );
}
