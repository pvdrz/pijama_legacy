use crate::test_type;

use pijama_ty::Ty;

use pijama_tycheck::TyErrorKind;

use pijama_driver::LangErrorKind;

test_type!(
    wrong_type_fn_call_arg,
    Err(&LangErrorKind::Ty(TyErrorKind::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool
    }))
);

test_type!(
    wrong_return_type_fn_int_to_int,
    Err(&LangErrorKind::Ty(TyErrorKind::Mismatch {
        expected: Ty::Bool,
        found: Ty::Int
    }))
);

test_type!(
    wrong_type_anon_fn_call_arg,
    Err(&LangErrorKind::Ty(TyErrorKind::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool
    }))
);

test_type!(
    wrong_type_rec_fn,
    Err(&LangErrorKind::Ty(TyErrorKind::Mismatch {
        expected: Ty::Unit,
        found: Ty::Int
    }))
);

test_type!(
    wrong_type_after_call,
    Err(&LangErrorKind::Ty(TyErrorKind::Mismatch {
        expected: Ty::Bool,
        found: Ty::Int,
    }))
);
