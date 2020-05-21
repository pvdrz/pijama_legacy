use crate::test_type;
use pijama::ty::Ty;

test_type!(shadowing_is_not_recursion, Ok(Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Int))));
test_type!(shadowing_is_not_recursion_2, Ok(Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Int))));
