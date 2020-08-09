use crate::{
    test_type_for_all_comparision_binops, test_type_for_all_equality_binops,
};

use pijama_ty::Ty;

use pijama_tycheck::TyErrorKind;

use pijama_driver::LangErrorKind;

// Test all int comparison operators with bool arguments
test_type_for_all_comparision_binops!(
    wrong_type_placeholder,
    Err(&LangErrorKind::Ty(TyErrorKind::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool
    })),
    OPERATOR
);

// Test all equality operators with int and bool arguments
test_type_for_all_equality_binops!(
    mixed_type_int_placeholder,
    Err(&LangErrorKind::Ty(TyErrorKind::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool
    })),
    OPERATOR
);

// Test all equality operators with bool and int arguments
test_type_for_all_equality_binops!(
    mixed_type_bool_placeholder,
    Err(&LangErrorKind::Ty(TyErrorKind::Mismatch {
        expected: Ty::Bool,
        found: Ty::Int
    })),
    OPERATOR
);
