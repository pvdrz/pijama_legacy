use crate::test_type;

use pijama_core::ty::Ty;

test_type!(cond_result_bool_is_bool, Ok(Ty::Bool));
test_type!(cond_result_int_is_int, Ok(Ty::Int));
