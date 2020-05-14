use crate::test_type;
use pijama::ty::{Ty, TyError};
use pijama::LangError;

test_type!(
    wrong_type_and,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Bool,
        found: Ty::Int
    }))
);
test_type!(
    wrong_type_not,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Bool,
        found: Ty::Int
    }))
);
