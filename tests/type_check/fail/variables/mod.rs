use crate::{test_type, util::DummyLoc};

use pijama::{ty::TyError, LangError};

test_type!(
    unbounded,
    Err(LangError::Ty(TyError::Unbounded("x".to_owned().loc())))
);
