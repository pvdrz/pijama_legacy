use crate::test_type;
use pijama::ty::{Ty, TyError};
use pijama::LangError;

test_type!(
    wrong_type_gt,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool
    }))
);
