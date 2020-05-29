use crate::test_type;
use pijama_ast::ty::Ty;

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
