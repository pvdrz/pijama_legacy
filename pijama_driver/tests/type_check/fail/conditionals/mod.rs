use crate::test_type;

use pijama_driver::LangErrorKind;
use pijama_ty::Ty;
use pijama_tycheck::TyErrorKind;

test_type!(
    wrong_type_cond_input,
    Err(&LangErrorKind::Ty(TyErrorKind::Mismatch {
        expected: Ty::Bool,
        found: Ty::Int
    }))
);
test_type!(
    mixed_types_cond_result,
    Err(&LangErrorKind::Ty(TyErrorKind::Mismatch {
        expected: Ty::Bool,
        found: Ty::Int
    }))
);
