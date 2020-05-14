use crate::test_type;
use pijama::ty::{Ty, TyError};
use pijama::LangError;

test_type!(
    wrong_type_minus,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool
    }))
);
test_type!(
    wrong_type_sub,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool
    }))
);
test_type!(
    mixed_types_add,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool
    }))
);
