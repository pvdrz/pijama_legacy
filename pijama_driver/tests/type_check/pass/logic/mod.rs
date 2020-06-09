use crate::test_type;

use pijama_ty::Ty;

// Unary operations
test_type!(not_is_bool, Ok(Ty::Bool));
// Binary operations
test_type!(and_is_bool, Ok(Ty::Bool));
test_type!(or_is_bool, Ok(Ty::Bool));
