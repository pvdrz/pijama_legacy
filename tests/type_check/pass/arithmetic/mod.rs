use crate::test_type;
use pijama::ty::Ty;

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
