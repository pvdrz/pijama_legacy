use crate::test_type;
use pijama::ty::{Ty, TyError};
use pijama::LangError;

test_type!(
    bind_bool_to_int,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool
    }))
);
