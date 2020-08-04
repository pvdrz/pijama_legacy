use crate::{test_type, util::DummyLoc};

use pijama_ty::Ty;

use pijama_tycheck::TyErrorKind;

use pijama_driver::LangError;

test_type!(
    wrong_type_fn_call_arg,
    Err(LangError::Ty(
        TyErrorKind::Mismatch {
            expected: Ty::Int,
            found: Ty::Bool
        }
        .loc()
    ))
);

test_type!(
    wrong_return_type_fn_int_to_int,
    Err(LangError::Ty(
        TyErrorKind::Mismatch {
            expected: Ty::Bool,
            found: Ty::Int
        }
        .loc()
    ))
);

test_type!(
    wrong_type_anon_fn_call_arg,
    Err(LangError::Ty(
        TyErrorKind::Mismatch {
            expected: Ty::Int,
            found: Ty::Bool
        }
        .loc()
    ))
);

test_type!(
    wrong_type_rec_fn,
    Err(LangError::Ty(
        TyErrorKind::Mismatch {
            expected: Ty::Unit,
            found: Ty::Int
        }
        .loc()
    ))
);

test_type!(
    wrong_type_after_call,
    Err(LangError::Ty(
        TyErrorKind::Mismatch {
            expected: Ty::Bool,
            found: Ty::Int,
        }
        .loc()
    ))
);
