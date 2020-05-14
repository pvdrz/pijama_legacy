use std::include_str;

use pijama::ty::{Ty, TyError};
use pijama::{mir, parser, ty, LangError, LangResult};
use super::type_check;

#[test]
fn wrong_cond_input_mismatch() {
    let input = include_str!("wrong_cond_input_mismatch.pj");
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
fn wrong_cond_result_mismatch() {
    let input = include_str!("wrong_cond_result_mismatch.pj");
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
