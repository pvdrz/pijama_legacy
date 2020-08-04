use crate::{test_type, util::DummyLoc};

use pijama_ty::Ty;

use pijama_tycheck::TyErrorKind;

use pijama_driver::LangError;

test_type!(
    unbounded,
    Err(LangError::Ty(TyErrorKind::Unbounded("x".to_owned()).loc()))
);

test_type!(
    shadowing,
    Err(LangError::Ty(
        TyErrorKind::Mismatch {
            expected: Ty::Int,
            found: Ty::Bool
        }
        .loc()
    ))
);
