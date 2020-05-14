use pijama::ty::Ty;
use pijama::{mir, parser, ty, LangResult};

mod error;

fn type_check(input: &str) -> LangResult<Ty> {
    let ast = parser::parse(input)?;
    let mir = mir::Term::from_ast(ast);
    ty::ty_check(&mir)
}

#[test]
fn true_is_bool() {
    let input = include_str!("true_is_bool.pj");
    let typ = type_check(input);
    assert!(matches!(typ, Ok(Ty::Bool)))
}

#[test]
fn false_is_bool() {
    let input = include_str!("false_is_bool.pj");
    let typ = type_check(input);
    assert!(matches!(typ, Ok(Ty::Bool)))
}

#[test]
fn number_is_int() {
    let input = include_str!("number_is_int.pj");
    let typ = type_check(input);
    assert!(matches!(typ, Ok(Ty::Int)))
}

#[test]
fn bool_eq_is_bool() {
    let input = include_str!("bool_eq_is_bool.pj");
    let typ = type_check(input);
    assert!(matches!(typ, Ok(Ty::Bool)))
}

#[test]
fn int_eq_is_bool() {
    let input = include_str!("int_eq_is_bool.pj");
    let typ = type_check(input);
    assert!(matches!(typ, Ok(Ty::Bool)))
}

#[test]
fn arith_op_is_int() {
    let input = include_str!("arith_op_is_int.pj");
    let typ = type_check(input);
    assert!(matches!(typ, Ok(Ty::Int)))
}

#[test]
fn logical_op_is_bool() {
    let input = include_str!("logical_op_is_bool.pj");
    let typ = type_check(input);
    assert!(matches!(typ, Ok(Ty::Bool)))
}

#[test]
fn int_cmp_is_bool() {
    let input = include_str!("int_cmp_is_bool.pj");
    let typ = type_check(input);
    assert!(matches!(typ, Ok(Ty::Bool)))
}

#[test]
fn negate_is_int() {
    let input = include_str!("negate_is_int.pj");
    let typ = type_check(input);
    assert!(matches!(typ, Ok(Ty::Int)))
}

#[test]
fn not_is_bool() {
    let input = include_str!("not_is_bool.pj");
    let typ = type_check(input);
    assert!(matches!(typ, Ok(Ty::Bool)))
}

#[test]
fn cond_result_bool_is_bool() {
    let input = include_str!("cond_result_bool_is_bool.pj");
    let typ = type_check(input);
    assert!(matches!(typ, Ok(Ty::Bool)))
}

#[test]
fn cond_result_int_is_int() {
    let input = include_str!("cond_result_int_is_int.pj");
    let typ = type_check(input);
    assert!(matches!(typ, Ok(Ty::Int)))
}