use crate::{
    test_type_for_all_comparision_binops, test_type_for_all_equality_binops, util::DummyLoc,
};

use pijama::{
    ty::{Ty, TyError},
    LangError,
};

// Test all int comparison operators with bool arguments
test_type_for_all_comparision_binops!(
    wrong_type_placeholder,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Int,
        found: Ty::Bool.loc()
    })),
    OPERATOR
);

// Test all equality operators with int and bool arguments
test_type_for_all_equality_binops!(
    mixed_type_int_placeholder,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Int,
        found: Ty::Bool.loc()
    })),
    OPERATOR
);

// Test all equality operators with bool and int arguments
test_type_for_all_equality_binops!(
    mixed_type_bool_placeholder,
    Err(LangError::Ty(TyError::Unexpected {
        expected: Ty::Bool,
        found: Ty::Int.loc()
    })),
    OPERATOR
);
