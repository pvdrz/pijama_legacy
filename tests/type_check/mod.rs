use pijama::ty::Ty;
use pijama::{mir, parser, ty, LangResult};

mod error;

fn type_check(input: &str) -> LangResult<Ty> {
    let ast = parser::parse(input)?;
    let mir = mir::Term::from_ast(ast);
    ty::ty_check(&mir)
}

macro_rules! test_type {
    ($name:ident, $pattern:pat) => {
        #[test]
        fn $name() {
            let input = include_str!(concat!(stringify!($name), ".pj"));
            let ty = type_check(input);
            assert!(matches!(ty, $pattern));
        }
    }
}

// Literals
mod literals {
    use super::*;
    test_type!(true_is_bool, Ok(Ty::Bool));
    test_type!(false_is_bool, Ok(Ty::Bool));
    test_type!(number_is_int, Ok(Ty::Int));
    test_type!(unit_is_unit, Ok(Ty::Unit));
}

// Non-recursive functions
test_type!(fn_from_int_to_int, Ok(Ty::Arrow(box Ty::Int, box Ty::Int)));

// Arithmetic operations
mod arithmetic {
    use super::*;
    // Unary operations
    test_type!(minus_is_int, Ok(Ty::Int));
    // Binary operations
    test_type!(add_is_int, Ok(Ty::Int));
    test_type!(sub_is_int, Ok(Ty::Int));
    test_type!(mul_is_int, Ok(Ty::Int));
    test_type!(div_is_int, Ok(Ty::Int));
    test_type!(mod_is_int, Ok(Ty::Int));
    test_type!(bitand_is_int, Ok(Ty::Int));
    test_type!(bitor_is_int, Ok(Ty::Int));
    test_type!(bitxor_is_int, Ok(Ty::Int));
}

// Logic operations
mod logic {
    use super::*;
    // Unary operations
    test_type!(not_is_bool, Ok(Ty::Bool));
    // Binary operations
    test_type!(and_is_bool, Ok(Ty::Bool));
    test_type!(or_is_bool, Ok(Ty::Bool));
}

// Comparison operations
mod comparison {
    use super::*;
    // Binary operations
    test_type!(lt_is_bool, Ok(Ty::Bool));
    test_type!(gt_is_bool, Ok(Ty::Bool));
    test_type!(leq_is_bool, Ok(Ty::Bool));
    test_type!(geq_is_bool, Ok(Ty::Bool));
    test_type!(bool_eq_is_bool, Ok(Ty::Bool));
    test_type!(int_eq_is_bool, Ok(Ty::Bool));
    test_type!(unit_eq_is_bool, Ok(Ty::Bool));
    test_type!(bool_neq_is_bool, Ok(Ty::Bool));
    test_type!(int_neq_is_bool, Ok(Ty::Bool));
    test_type!(unit_neq_is_bool, Ok(Ty::Bool));
}

mod conditionals {
    use super::*;
    test_type!(cond_result_bool_is_bool, Ok(Ty::Bool));
    test_type!(cond_result_int_is_int, Ok(Ty::Int));
}
