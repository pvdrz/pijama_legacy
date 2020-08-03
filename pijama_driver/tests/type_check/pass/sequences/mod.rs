use crate::test_type;

use pijama_ty::Ty;

test_type!(unit_can_be_ignored, Ok(Ty::Bool));
test_type!(int_can_be_ignored, Ok(Ty::Bool));
