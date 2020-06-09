use crate::{test_type, util::DummyLoc};

use pijama_ty::Ty;

use pijama_tycheck::TyError;

use pijama_driver::LangError;

test_type!(
    int_cannot_be_ignored,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Unit,
        found: Ty::Int.loc()
    }))
);
