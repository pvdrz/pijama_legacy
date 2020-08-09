use crate::test_type;

use pijama_driver::LangErrorKind;
use pijama_hir::LowerErrorKind;
use pijama_ty::Ty;
use pijama_tycheck::TyErrorKind;

test_type!(
    unbounded,
    Err(&LangErrorKind::Lower(LowerErrorKind::Unbounded(
        "x".to_owned()
    )))
);

test_type!(
    shadowing,
    Err(&LangErrorKind::Ty(TyErrorKind::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool
    }))
);
