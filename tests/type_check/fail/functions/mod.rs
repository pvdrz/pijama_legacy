use crate::test_type;
use pijama::ty::{Ty, TyError};
use pijama::LangError;

test_type!(
    wrong_type_fn_call_arg,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool
    }))
);
