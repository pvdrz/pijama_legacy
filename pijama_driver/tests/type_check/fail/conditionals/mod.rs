use crate::{test_type, util::DummyLoc};

use pijama_ty::Ty;

use pijama_tycheck::TyErrorKind;

use pijama_driver::LangError;

test_type!(
    wrong_type_cond_input,
    Err(LangError::Ty(
        TyErrorKind::Mismatch {
            expected: Ty::Bool,
            found: Ty::Int
        }
        .loc()
    ))
);
test_type!(
    mixed_types_cond_result,
    Err(LangError::Ty(
        TyErrorKind::Mismatch {
            expected: Ty::Bool,
            found: Ty::Int
        }
        .loc()
    ))
);
