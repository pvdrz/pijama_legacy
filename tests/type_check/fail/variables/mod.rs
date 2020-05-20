use crate::test_type;
use pijama::{ty::TyError, LangError};

test_type!(unbounded, Err(LangError::Ty(TyError::Unbound(_))));
