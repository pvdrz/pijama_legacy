use crate::test_type;
use pijama::ty::{Ty, TyError};
use pijama::LangError;

test_type!(
    wrong_type_cond_input,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Bool,
        found: Ty::Int
    }))
);
test_type!(
    mixed_types_cond_result,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Bool,
        found: Ty::Int
    }))
);
