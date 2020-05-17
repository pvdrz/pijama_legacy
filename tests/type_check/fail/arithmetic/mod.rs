use crate::{test_type, test_type_for_all_integer_binops};
use pijama::ty::{Ty, TyError};
use pijama::LangError;

test_type!(
    wrong_type_minus,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool
    }))
);

// Test all int binary operators with a bool and a int argument
test_type_for_all_integer_binops!(
    mixed_types_placeholder,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool
    })),
    OPERATOR
);

// Test all int binary operators with bool arguments
test_type_for_all_integer_binops!(
    wrong_type_placeholder,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool
    })),
    OPERATOR
);
