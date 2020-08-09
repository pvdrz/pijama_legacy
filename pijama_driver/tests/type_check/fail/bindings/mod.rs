use crate::test_type;

use pijama_driver::LangErrorKind;
use pijama_ty::Ty;
use pijama_tycheck::TyErrorKind;

test_type!(
    bind_bool_to_int,
    Err(&LangErrorKind::Ty(TyErrorKind::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool
    }))
);
