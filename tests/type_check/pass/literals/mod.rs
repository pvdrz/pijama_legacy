use crate::test_type;
use pijama_ast::ty::Ty;

test_type!(true_is_bool, Ok(Ty::Bool));
test_type!(false_is_bool, Ok(Ty::Bool));
test_type!(number_is_int, Ok(Ty::Int));
test_type!(unit_is_unit, Ok(Ty::Unit));
