use crate::{test_type, util::DummyLoc};

use pijama_common::location::Location;

use pijama_ty::Ty;

use pijama_tycheck::TyErrorKind;

use pijama_hir::LowerError;

use pijama_driver::LangError;

test_type!(
    unbounded,
    Err(LangError::Lower(LowerError::Unbounded(
        "x".to_owned(),
        Location::new(0, 0)
    )))
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
