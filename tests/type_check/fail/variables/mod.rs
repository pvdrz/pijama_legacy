use crate::test_type;
use pijama::ty::{Ty, TyError};
use pijama::LangError;

test_type!(unbounded, Err(LangError::Ty(TyError::Unbound(_))));
