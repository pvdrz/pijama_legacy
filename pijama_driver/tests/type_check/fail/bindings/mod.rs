use crate::{test_type, util::DummyLoc};

use pijama_ty::Ty;

use pijama_tycheck::TyError;

use pijama_driver::LangError;

test_type!(
    bind_bool_to_int,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool.loc()
    }))
);
