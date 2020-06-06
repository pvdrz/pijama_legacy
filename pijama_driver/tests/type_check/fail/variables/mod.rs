use crate::{test_type, util::DummyLoc};

use pijama_core::ty::TyError;
use pijama_driver::LangError;

test_type!(
    unbounded,
    Err(LangError::Ty(TyError::Unbounded("x".to_owned().loc())))
);
