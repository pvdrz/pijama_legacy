use crate::{test_type, util::DummyLoc};

use pijama_core::ty::{Ty, TyError};
use pijama_driver::LangError;

test_type!(
    unbounded,
    Err(LangError::Ty(TyError::Unbounded("x".to_owned().loc())))
);

test_type!(
    shadowing,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Int,
        found: Ty::Bool.loc()
    }))
);
