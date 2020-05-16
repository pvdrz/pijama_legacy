use crate::{test_type, test_type_with_placeholder};
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
    mixed_types_add,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool
    }))
);

// Test all int binary operators with bool arguments
test_type_with_placeholder!(
    wrong_type_int_binop_with_placeholder,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool
    })),
    OPERATOR,
    /, *, +, -, &, |, ^
);
