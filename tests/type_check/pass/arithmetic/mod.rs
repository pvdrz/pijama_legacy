use crate::{test_type, test_type_for_all_integer_binops};
use pijama_ast::ty::Ty;

// Unary operations
test_type!(minus_is_int, Ok(Ty::Int));

// Test all int binary operators with int arguments
test_type_for_all_integer_binops!(int_binop_with_placeholder, Ok(Ty::Int), OPERATOR);
